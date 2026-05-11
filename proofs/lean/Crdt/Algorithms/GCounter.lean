import Crdt.StateBased.CvRDT

namespace Crdt
namespace GCounter

abbrev State (Actor : Type u) := Actor -> Nat

def join (left right : State Actor) : State Actor :=
  fun actor => Nat.max (left actor) (right actor)

instance : JoinSemilattice (State Actor) where
  join := join
  join_assoc := by
    intro a b c
    funext actor
    simp [join, Nat.max_assoc]
  join_comm := by
    intro a b
    funext actor
    simp [join, Nat.max_comm]
  join_idem := by
    intro a
    funext actor
    simp [join]

def zero : State Actor :=
  fun _ => 0

def singleton [DecidableEq Actor] (actor : Actor) (amount : Nat) : State Actor :=
  fun current => if current = actor then amount else 0

def incrementBy [DecidableEq Actor] (actor : Actor) (amount : Nat)
    (state : State Actor) : State Actor :=
  fun current => if current = actor then state current + amount else state current

def increment [DecidableEq Actor] (actor : Actor) (state : State Actor) : State Actor :=
  incrementBy actor 1 state

inductive Update (Actor : Type u) where
  | incrementBy (actor : Actor) (amount : Nat)

def applyUpdate [DecidableEq Actor] (update : Update Actor)
    (state : State Actor) : State Actor :=
  match update with
  | Update.incrementBy actor amount => incrementBy actor amount state

theorem incrementBy_inflationary [DecidableEq Actor] (actor : Actor)
    (amount : Nat) (state : State Actor) :
    leq state (incrementBy actor amount state) := by
  unfold leq
  change join state (incrementBy actor amount state) = incrementBy actor amount state
  unfold incrementBy
  funext current
  by_cases h : current = actor
  · simp [join, h]
  · simp [join, h]

theorem increment_inflationary [DecidableEq Actor] (actor : Actor) (state : State Actor) :
    leq state (increment actor state) := by
  unfold increment
  exact incrementBy_inflationary actor 1 state

theorem applyUpdate_inflationary [DecidableEq Actor] (update : Update Actor)
    (state : State Actor) :
    leq state (applyUpdate update state) := by
  cases update with
  | incrementBy actor amount =>
      exact incrementBy_inflationary actor amount state

instance [DecidableEq Actor] : CvRDT (State Actor) where
  Update := Update Actor
  Query := State Actor
  apply := applyUpdate
  query := fun state => state
  apply_inflationary := applyUpdate_inflationary

theorem merge_monotone {leftBefore leftAfter rightBefore rightAfter : State Actor} :
    leq leftBefore leftAfter ->
    leq rightBefore rightAfter ->
    leq
      (JoinSemilattice.join leftBefore rightBefore)
      (JoinSemilattice.join leftAfter rightAfter) := by
  exact join_monotone

theorem merge_converges_for_same_states (left right : State Actor) :
    JoinSemilattice.join left right = JoinSemilattice.join right left :=
  JoinSemilattice.join_comm left right

end GCounter
end Crdt
