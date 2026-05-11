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

theorem assign_inflationary (value : Nat) (state : State) :
    leq state (assign value state) := by
  unfold assign
  exact leq_join_left state (some value)

theorem merge_converges_for_same_states (left right : State) :
    JoinSemilattice.join left right = JoinSemilattice.join right left :=
  JoinSemilattice.join_comm left right

end MaxRegister
end Crdt
