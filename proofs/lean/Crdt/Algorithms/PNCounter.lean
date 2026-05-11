import Crdt.Algorithms.GCounter

namespace Crdt
namespace PNCounter

structure State (Actor : Type u) where
  increments : GCounter.State Actor
  decrements : GCounter.State Actor

theorem state_ext {left right : State Actor}
    (increments_eq : left.increments = right.increments)
    (decrements_eq : left.decrements = right.decrements) :
    left = right := by
  cases left
  cases right
  simp at increments_eq decrements_eq
  simp [increments_eq, decrements_eq]

def join (left right : State Actor) : State Actor :=
  {
    increments := GCounter.join left.increments right.increments
    decrements := GCounter.join left.decrements right.decrements
  }

instance : JoinSemilattice (State Actor) where
  join := join
  join_assoc := by
    intro a b c
    apply state_ext <;> funext actor <;>
      simp [join, GCounter.join, Nat.max_assoc]
  join_comm := by
    intro a b
    apply state_ext <;> funext actor <;>
      simp [join, GCounter.join, Nat.max_comm]
  join_idem := by
    intro a
    apply state_ext <;> funext actor <;>
      simp [join, GCounter.join]

def zero : State Actor :=
  {
    increments := GCounter.zero
    decrements := GCounter.zero
  }

def incrementBy [DecidableEq Actor] (actor : Actor) (amount : Nat)
    (state : State Actor) : State Actor :=
  { state with increments := GCounter.incrementBy actor amount state.increments }

def decrementBy [DecidableEq Actor] (actor : Actor) (amount : Nat)
    (state : State Actor) : State Actor :=
  { state with decrements := GCounter.incrementBy actor amount state.decrements }

def increment [DecidableEq Actor] (actor : Actor) (state : State Actor) : State Actor :=
  incrementBy actor 1 state

def decrement [DecidableEq Actor] (actor : Actor) (state : State Actor) : State Actor :=
  decrementBy actor 1 state

inductive Update (Actor : Type u) where
  | incrementBy (actor : Actor) (amount : Nat)
  | decrementBy (actor : Actor) (amount : Nat)

def applyUpdate [DecidableEq Actor] (update : Update Actor)
    (state : State Actor) : State Actor :=
  match update with
  | Update.incrementBy actor amount => incrementBy actor amount state
  | Update.decrementBy actor amount => decrementBy actor amount state

theorem incrementBy_inflationary [DecidableEq Actor] (actor : Actor)
    (amount : Nat) (state : State Actor) :
    leq state (incrementBy actor amount state) := by
  unfold leq incrementBy
  apply state_ext
  · change
      GCounter.join state.increments
        (GCounter.incrementBy actor amount state.increments)
        = GCounter.incrementBy actor amount state.increments
    exact GCounter.incrementBy_inflationary actor amount state.increments
  · change GCounter.join state.decrements state.decrements = state.decrements
    exact JoinSemilattice.join_idem state.decrements

theorem decrementBy_inflationary [DecidableEq Actor] (actor : Actor)
    (amount : Nat) (state : State Actor) :
    leq state (decrementBy actor amount state) := by
  unfold leq decrementBy
  apply state_ext
  · change GCounter.join state.increments state.increments = state.increments
    exact JoinSemilattice.join_idem state.increments
  · change
      GCounter.join state.decrements
        (GCounter.incrementBy actor amount state.decrements)
        = GCounter.incrementBy actor amount state.decrements
    exact GCounter.incrementBy_inflationary actor amount state.decrements

theorem increment_inflationary [DecidableEq Actor] (actor : Actor)
    (state : State Actor) :
    leq state (increment actor state) := by
  unfold increment
  exact incrementBy_inflationary actor 1 state

theorem decrement_inflationary [DecidableEq Actor] (actor : Actor)
    (state : State Actor) :
    leq state (decrement actor state) := by
  unfold decrement
  exact decrementBy_inflationary actor 1 state

theorem applyUpdate_inflationary [DecidableEq Actor] (update : Update Actor)
    (state : State Actor) :
    leq state (applyUpdate update state) := by
  cases update with
  | incrementBy actor amount =>
      exact incrementBy_inflationary actor amount state
  | decrementBy actor amount =>
      exact decrementBy_inflationary actor amount state

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

end PNCounter
end Crdt
