import Crdt.Algorithms.MaxRegister

namespace Crdt
namespace LWWElementSet

abbrev Timestamp := Option Nat

@[simp]
theorem timestamp_join_none_some (timestamp : Nat) :
    JoinSemilattice.join (none : Timestamp) (some timestamp) = some timestamp := by
  rfl

@[simp]
theorem timestamp_join_some_some (left right : Nat) :
    JoinSemilattice.join (some left : Timestamp) (some right) =
      some (Nat.max left right) := by
  rfl

abbrev Component (Element : Type u) := Element -> Timestamp

def componentJoin (left right : Component Element) : Component Element :=
  fun element => JoinSemilattice.join (left element) (right element)

instance : JoinSemilattice (Component Element) where
  join := componentJoin
  join_assoc := by
    intro a b c
    funext element
    exact JoinSemilattice.join_assoc (a element) (b element) (c element)
  join_comm := by
    intro a b
    funext element
    exact JoinSemilattice.join_comm (a element) (b element)
  join_idem := by
    intro a
    funext element
    exact JoinSemilattice.join_idem (a element)

structure State (Element : Type u) where
  adds : Component Element
  removes : Component Element

theorem state_ext {left right : State Element}
    (adds_eq : left.adds = right.adds)
    (removes_eq : left.removes = right.removes) :
    left = right := by
  cases left
  cases right
  simp at adds_eq removes_eq
  simp [adds_eq, removes_eq]

def join (left right : State Element) : State Element :=
  {
    adds := componentJoin left.adds right.adds
    removes := componentJoin left.removes right.removes
  }

instance : JoinSemilattice (State Element) where
  join := join
  join_assoc := by
    intro a b c
    apply state_ext
    · exact JoinSemilattice.join_assoc a.adds b.adds c.adds
    · exact JoinSemilattice.join_assoc a.removes b.removes c.removes
  join_comm := by
    intro a b
    apply state_ext
    · exact JoinSemilattice.join_comm a.adds b.adds
    · exact JoinSemilattice.join_comm a.removes b.removes
  join_idem := by
    intro a
    apply state_ext
    · exact JoinSemilattice.join_idem a.adds
    · exact JoinSemilattice.join_idem a.removes

def empty : State Element :=
  {
    adds := fun _ => none
    removes := fun _ => none
  }

def singleton [DecidableEq Element] (element : Element) (timestamp : Nat) :
    Component Element :=
  fun current => if current = element then some timestamp else none

def add [DecidableEq Element] (element : Element) (timestamp : Nat)
    (state : State Element) : State Element :=
  { state with adds := componentJoin state.adds (singleton element timestamp) }

def remove [DecidableEq Element] (element : Element) (timestamp : Nat)
    (state : State Element) : State Element :=
  { state with removes := componentJoin state.removes (singleton element timestamp) }

def visible (addTimestamp removeTimestamp : Timestamp) : Bool :=
  match addTimestamp, removeTimestamp with
  | none, _ => false
  | some _, none => true
  | some addTimestamp, some removeTimestamp => decide (removeTimestamp < addTimestamp)

def contains (state : State Element) (element : Element) : Bool :=
  visible (state.adds element) (state.removes element)

theorem add_inflationary [DecidableEq Element] (element : Element)
    (timestamp : Nat) (state : State Element) :
    leq state (add element timestamp state) := by
  unfold leq
  change join state (add element timestamp state) = add element timestamp state
  unfold add join
  apply state_ext
  · change
      componentJoin state.adds
        (componentJoin state.adds (singleton element timestamp))
        = componentJoin state.adds (singleton element timestamp)
    exact leq_join_left state.adds (singleton element timestamp)
  · change componentJoin state.removes state.removes = state.removes
    exact JoinSemilattice.join_idem state.removes

theorem remove_inflationary [DecidableEq Element] (element : Element)
    (timestamp : Nat) (state : State Element) :
    leq state (remove element timestamp state) := by
  unfold leq
  change join state (remove element timestamp state) = remove element timestamp state
  unfold remove join
  apply state_ext
  · change componentJoin state.adds state.adds = state.adds
    exact JoinSemilattice.join_idem state.adds
  · change
      componentJoin state.removes
        (componentJoin state.removes (singleton element timestamp))
        = componentJoin state.removes (singleton element timestamp)
    exact leq_join_left state.removes (singleton element timestamp)

theorem added_visible_without_remove [DecidableEq Element] (element : Element)
    (timestamp : Nat) :
    contains (add element timestamp (empty : State Element)) element = true := by
  unfold contains visible add empty singleton componentJoin
  simp

theorem equal_timestamps_remove_win [DecidableEq Element] (element : Element)
    (timestamp : Nat) :
    contains
      (remove element timestamp (add element timestamp (empty : State Element)))
      element = false := by
  unfold contains visible remove add empty singleton componentJoin
  simp

theorem older_add_timestamp_ignored [DecidableEq Element] (element : Element)
    {older current : Nat} (older_le_current : older ≤ current)
    (state : State Element)
    (current_observed : state.adds element = some current) :
    (add element older state).adds element = some current := by
  unfold add singleton componentJoin
  simp [current_observed, Nat.max_eq_left older_le_current]

theorem older_remove_timestamp_ignored [DecidableEq Element] (element : Element)
    {older current : Nat} (older_le_current : older ≤ current)
    (state : State Element)
    (current_observed : state.removes element = some current) :
    (remove element older state).removes element = some current := by
  unfold remove singleton componentJoin
  simp [current_observed, Nat.max_eq_left older_le_current]

theorem merge_monotone {leftBefore leftAfter rightBefore rightAfter : State Element} :
    leq leftBefore leftAfter ->
    leq rightBefore rightAfter ->
    leq
      (JoinSemilattice.join leftBefore rightBefore)
      (JoinSemilattice.join leftAfter rightAfter) := by
  exact join_monotone

theorem merge_converges_for_same_states (left right : State Element) :
    JoinSemilattice.join left right = JoinSemilattice.join right left :=
  JoinSemilattice.join_comm left right

end LWWElementSet
end Crdt
