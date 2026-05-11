import Crdt.StateBased.CvRDT

namespace Crdt
namespace MaxRegister

abbrev State := Option Nat

def join : State -> State -> State
  | none, right => right
  | left, none => left
  | some left, some right => some (Nat.max left right)

instance : JoinSemilattice State where
  join := join
  join_assoc := by
    intro a b c
    cases a <;> cases b <;> cases c <;> simp [join, Nat.max_assoc]
  join_comm := by
    intro a b
    cases a <;> cases b <;> simp [join, Nat.max_comm]
  join_idem := by
    intro a
    cases a <;> simp [join]

def bottom : State :=
  none

def assign (value : Nat) (state : State) : State :=
  JoinSemilattice.join state (some value)

inductive Update where
  | assign (value : Nat)

def applyUpdate (update : Update) (state : State) : State :=
  match update with
  | Update.assign value => assign value state

theorem assign_inflationary (value : Nat) (state : State) :
    leq state (assign value state) := by
  unfold assign
  exact leq_join_left state (some value)

theorem applyUpdate_inflationary (update : Update) (state : State) :
    leq state (applyUpdate update state) := by
  cases update with
  | assign value => exact assign_inflationary value state

instance : CvRDT State where
  Update := Update
  Query := State
  apply := applyUpdate
  query := fun state => state
  apply_inflationary := applyUpdate_inflationary

theorem merge_monotone {leftBefore leftAfter rightBefore rightAfter : State} :
    leq leftBefore leftAfter ->
    leq rightBefore rightAfter ->
    leq
      (JoinSemilattice.join leftBefore rightBefore)
      (JoinSemilattice.join leftAfter rightAfter) := by
  exact join_monotone

theorem merge_converges_for_same_states (left right : State) :
    JoinSemilattice.join left right = JoinSemilattice.join right left :=
  JoinSemilattice.join_comm left right

end MaxRegister
end Crdt
