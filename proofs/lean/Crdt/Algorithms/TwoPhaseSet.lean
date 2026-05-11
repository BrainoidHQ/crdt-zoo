import Crdt.Algorithms.GSet

namespace Crdt
namespace TwoPhaseSet

structure State (Element : Type u) where
  adds : GSet.State Element
  removes : GSet.State Element

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
    adds := GSet.join left.adds right.adds
    removes := GSet.join left.removes right.removes
  }

instance : JoinSemilattice (State Element) where
  join := join
  join_assoc := by
    intro a b c
    apply state_ext <;> funext element <;>
      simp [join, GSet.join, Bool.or_assoc]
  join_comm := by
    intro a b
    apply state_ext <;> funext element <;>
      simp [join, GSet.join, Bool.or_comm]
  join_idem := by
    intro a
    apply state_ext <;> funext element <;>
      simp [join, GSet.join, Bool.or_self]

def empty : State Element :=
  {
    adds := GSet.empty
    removes := GSet.empty
  }

def contains (state : State Element) (element : Element) : Bool :=
  state.adds element && !state.removes element

def add [DecidableEq Element] (element : Element) (state : State Element) :
    State Element :=
  if state.removes element then
    state
  else
    { state with adds := GSet.add element state.adds }

def remove [DecidableEq Element] (element : Element) (state : State Element) :
    State Element :=
  { state with removes := GSet.add element state.removes }

theorem add_inflationary [DecidableEq Element] (element : Element)
    (state : State Element) :
    leq state (add element state) := by
  unfold add
  cases h : state.removes element
  · simp
    unfold leq
    apply state_ext
    · change GSet.join state.adds (GSet.add element state.adds)
        = GSet.add element state.adds
      exact GSet.add_inflationary element state.adds
    · change GSet.join state.removes state.removes = state.removes
      exact JoinSemilattice.join_idem state.removes
  · simp
    exact leq_refl state

theorem remove_inflationary [DecidableEq Element] (element : Element)
    (state : State Element) :
    leq state (remove element state) := by
  unfold leq remove
  apply state_ext
  · change GSet.join state.adds state.adds = state.adds
    exact JoinSemilattice.join_idem state.adds
  · change GSet.join state.removes (GSet.add element state.removes)
      = GSet.add element state.removes
    exact GSet.add_inflationary element state.removes

theorem remove_wins [DecidableEq Element] (element : Element)
    (state : State Element) :
    contains (remove element state) element = false := by
  unfold contains remove
  have hRemoved : GSet.add element state.removes element = true :=
    GSet.contains_added element state.removes
  simp [hRemoved]

theorem add_after_remove_noop [DecidableEq Element] (element : Element)
    (state : State Element) :
    add element (remove element state) = remove element state := by
  unfold add
  have hRemoved : (remove element state).removes element = true := by
    unfold remove
    exact GSet.contains_added element state.removes
  simp [hRemoved]

theorem merge_converges_for_same_states (left right : State Element) :
    JoinSemilattice.join left right = JoinSemilattice.join right left :=
  JoinSemilattice.join_comm left right

end TwoPhaseSet
end Crdt
