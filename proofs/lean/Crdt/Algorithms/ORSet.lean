import Crdt.Algebra.JoinSemilattice

namespace Crdt
namespace ORSet

abbrev DotSet (Dot : Type v) := Dot -> Bool
abbrev Entries (Element : Type u) (Dot : Type v) := Element -> Dot -> Bool

def dotSetJoin (left right : DotSet Dot) : DotSet Dot :=
  fun dot => left dot || right dot

def mergeEntry (leftEntry rightEntry leftContext rightContext : Bool) : Bool :=
  (leftEntry && (rightEntry || !rightContext)) ||
    (rightEntry && (leftEntry || !leftContext))

theorem mergeEntry_comm (leftEntry rightEntry leftContext rightContext : Bool) :
    mergeEntry leftEntry rightEntry leftContext rightContext =
      mergeEntry rightEntry leftEntry rightContext leftContext := by
  cases leftEntry <;> cases rightEntry <;>
    cases leftContext <;> cases rightContext <;>
    simp [mergeEntry]

theorem mergeEntry_idem (entry context : Bool) :
    mergeEntry entry entry context context = entry := by
  cases entry <;> cases context <;> simp [mergeEntry]

theorem mergeEntry_observed {leftEntry rightEntry leftContext rightContext : Bool}
    (leftObserved : leftEntry = true -> leftContext = true)
    (rightObserved : rightEntry = true -> rightContext = true) :
    mergeEntry leftEntry rightEntry leftContext rightContext = true ->
      (leftContext || rightContext) = true := by
  cases leftEntry <;> cases rightEntry <;>
    cases leftContext <;> cases rightContext <;>
    simp [mergeEntry] at *

theorem mergeEntry_assoc {leftEntry rightEntry thirdEntry leftContext rightContext thirdContext : Bool}
    (leftObserved : leftEntry = true -> leftContext = true)
    (rightObserved : rightEntry = true -> rightContext = true)
    (thirdObserved : thirdEntry = true -> thirdContext = true) :
    mergeEntry
        (mergeEntry leftEntry rightEntry leftContext rightContext)
        thirdEntry
        (leftContext || rightContext)
        thirdContext =
      mergeEntry
        leftEntry
        (mergeEntry rightEntry thirdEntry rightContext thirdContext)
        leftContext
        (rightContext || thirdContext) := by
  cases leftEntry <;> cases rightEntry <;> cases thirdEntry <;>
    cases leftContext <;> cases rightContext <;> cases thirdContext <;>
    simp [mergeEntry] at *

structure State (Element : Type u) (Dot : Type v) where
  entries : Entries Element Dot
  context : DotSet Dot
  observed : forall element dot, entries element dot = true -> context dot = true

theorem state_ext {left right : State Element Dot}
    (entries_eq : left.entries = right.entries)
    (context_eq : left.context = right.context) :
    left = right := by
  cases left
  cases right
  simp at entries_eq context_eq
  simp [entries_eq, context_eq]

def empty : State Element Dot :=
  {
    entries := fun _ _ => false
    context := fun _ => false
    observed := by
      intro _ _ h
      contradiction
  }

def singletonEntries [DecidableEq Element] [DecidableEq Dot]
    (element : Element) (dot : Dot) : Entries Element Dot :=
  fun current candidate => decide (current = element && candidate = dot)

def add [DecidableEq Element] [DecidableEq Dot] (element : Element) (dot : Dot)
    (state : State Element Dot) : State Element Dot :=
  {
    entries :=
      fun current candidate =>
        state.entries current candidate || singletonEntries element dot current candidate
    context := fun candidate => state.context candidate || decide (candidate = dot)
    observed := by
      intro current candidate hEntry
      unfold singletonEntries at hEntry
      by_cases hStateEntry : state.entries current candidate = true
      · have hObserved := state.observed current candidate hStateEntry
        simp [hObserved]
      · by_cases hCandidate : candidate = dot
        · simp [hCandidate]
        · simp [hStateEntry, hCandidate] at hEntry
  }

def remove [DecidableEq Element] (element : Element)
    (state : State Element Dot) : State Element Dot :=
  {
    entries :=
      fun current dot =>
        if current = element then false else state.entries current dot
    context := state.context
    observed := by
      intro current dot hEntry
      by_cases hCurrent : current = element
      · simp [hCurrent] at hEntry
      · exact state.observed current dot (by simpa [hCurrent] using hEntry)
  }

def join (left right : State Element Dot) : State Element Dot :=
  {
    entries :=
      fun element dot =>
        mergeEntry
          (left.entries element dot)
          (right.entries element dot)
          (left.context dot)
          (right.context dot)
    context := dotSetJoin left.context right.context
    observed := by
      intro element dot hEntry
      unfold dotSetJoin
      exact mergeEntry_observed
        (left.observed element dot)
        (right.observed element dot)
        hEntry
  }

instance : JoinSemilattice (State Element Dot) where
  join := join
  join_assoc := by
    intro a b c
    apply state_ext
    · funext element dot
      unfold join dotSetJoin
      exact mergeEntry_assoc
        (a.observed element dot)
        (b.observed element dot)
        (c.observed element dot)
    · funext dot
      unfold join dotSetJoin
      exact Bool.or_assoc (a.context dot) (b.context dot) (c.context dot)
  join_comm := by
    intro a b
    apply state_ext
    · funext element dot
      unfold join
      exact mergeEntry_comm
        (a.entries element dot)
        (b.entries element dot)
        (a.context dot)
        (b.context dot)
    · funext dot
      unfold join dotSetJoin
      exact Bool.or_comm (a.context dot) (b.context dot)
  join_idem := by
    intro state
    apply state_ext
    · funext element dot
      unfold join
      exact mergeEntry_idem (state.entries element dot) (state.context dot)
    · funext dot
      unfold join dotSetJoin
      exact Bool.or_self (state.context dot)

def containsDot (state : State Element Dot) (element : Element) (dot : Dot) : Bool :=
  state.entries element dot

theorem add_contains_dot [DecidableEq Element] [DecidableEq Dot]
    (element : Element) (dot : Dot) (state : State Element Dot) :
    containsDot (add element dot state) element dot = true := by
  unfold containsDot add singletonEntries
  simp

theorem add_observes_dot [DecidableEq Element] [DecidableEq Dot]
    (element : Element) (dot : Dot) (state : State Element Dot) :
    (add element dot state).context dot = true := by
  unfold add
  simp

theorem add_inflationary [DecidableEq Element] [DecidableEq Dot]
    (element : Element) (dot : Dot) (state : State Element Dot)
    (fresh : state.context dot = false) :
    leq state (add element dot state) := by
  unfold leq
  change join state (add element dot state) = add element dot state
  apply state_ext
  · funext current candidate
    unfold join add singletonEntries
    by_cases hCandidate : candidate = dot
    · by_cases hCurrent : current = element
      · simp [mergeEntry, hCandidate, hCurrent, fresh]
      · simp [mergeEntry, hCandidate, hCurrent, fresh]
    · by_cases hStateEntry : state.entries current candidate = true
      · have hObserved := state.observed current candidate hStateEntry
        simp [mergeEntry, hCandidate, hStateEntry, hObserved]
      · simp [mergeEntry, hCandidate, hStateEntry]
  · funext candidate
    unfold join add dotSetJoin
    by_cases hCandidate : candidate = dot
    · simp [hCandidate]
    · simp [hCandidate, Bool.or_self]

theorem remove_inflationary [DecidableEq Element]
    (element : Element) (state : State Element Dot) :
    leq state (remove element state) := by
  unfold leq
  change join state (remove element state) = remove element state
  apply state_ext
  · funext current dot
    unfold join remove dotSetJoin
    by_cases hCurrent : current = element
    · subst current
      cases hStateEntry : state.entries element dot
      · simp [mergeEntry, hStateEntry]
      · have hObserved := state.observed element dot hStateEntry
        simp [mergeEntry, hStateEntry, hObserved]
    · by_cases hStateEntry : state.entries current dot = true
      · have hObserved := state.observed current dot hStateEntry
        simp [mergeEntry, hCurrent, hStateEntry, hObserved]
      · simp [mergeEntry, hCurrent, hStateEntry]
  · funext dot
    unfold join remove dotSetJoin
    exact Bool.or_self (state.context dot)

theorem observed_remove_clears_dot [DecidableEq Element]
    (element : Element) (dot : Dot) (state : State Element Dot) :
    containsDot (remove element state) element dot = false := by
  unfold containsDot remove
  simp

theorem concurrent_add_wins [DecidableEq Element] [DecidableEq Dot]
    (element : Element) (dot : Dot)
    (added removed : State Element Dot)
    (hAddedEntry : added.entries element dot = true)
    (hRemovedMissesDot : removed.context dot = false) :
    containsDot (JoinSemilattice.join added removed) element dot = true := by
  unfold containsDot
  change (join added removed).entries element dot = true
  unfold join
  simp [mergeEntry, hAddedEntry, hRemovedMissesDot]

theorem merge_monotone {leftBefore leftAfter rightBefore rightAfter : State Element Dot} :
    leq leftBefore leftAfter ->
    leq rightBefore rightAfter ->
    leq
      (JoinSemilattice.join leftBefore rightBefore)
      (JoinSemilattice.join leftAfter rightAfter) := by
  exact join_monotone

theorem merge_converges_for_same_states (left right : State Element Dot) :
    JoinSemilattice.join left right = JoinSemilattice.join right left :=
  JoinSemilattice.join_comm left right

end ORSet
end Crdt
