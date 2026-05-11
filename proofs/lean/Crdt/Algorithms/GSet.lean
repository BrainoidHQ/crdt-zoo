import Crdt.Algorithms.BoolOr

namespace Crdt
namespace GSet

abbrev State (Element : Type u) := Element -> Bool

def join (left right : State Element) : State Element :=
  fun element => left element || right element

instance : JoinSemilattice (State Element) where
  join := join
  join_assoc := by
    intro a b c
    funext element
    exact Bool.or_assoc (a element) (b element) (c element)
  join_comm := by
    intro a b
    funext element
    exact Bool.or_comm (a element) (b element)
  join_idem := by
    intro a
    funext element
    exact Bool.or_self (a element)

def empty : State Element :=
  fun _ => false

def singleton [DecidableEq Element] (element : Element) : State Element :=
  fun current => if current = element then true else false

def contains (state : State Element) (element : Element) : Bool :=
  state element

def add [DecidableEq Element] (element : Element) (state : State Element) :
    State Element :=
  JoinSemilattice.join state (singleton element)

inductive Update (Element : Type u) where
  | add (element : Element)

def applyUpdate [DecidableEq Element] (update : Update Element)
    (state : State Element) : State Element :=
  match update with
  | Update.add element => add element state

theorem add_inflationary [DecidableEq Element] (element : Element)
    (state : State Element) :
    leq state (add element state) := by
  unfold add
  exact leq_join_left state (singleton element)

theorem contains_added [DecidableEq Element] (element : Element)
    (state : State Element) :
    contains (add element state) element = true := by
  unfold contains add
  change join state (singleton element) element = true
  unfold singleton
  simp [join]

theorem applyUpdate_inflationary [DecidableEq Element] (update : Update Element)
    (state : State Element) :
    leq state (applyUpdate update state) := by
  cases update with
  | add element => exact add_inflationary element state

instance [DecidableEq Element] : CvRDT (State Element) where
  Update := Update Element
  Query := State Element
  apply := applyUpdate
  query := fun state => state
  apply_inflationary := applyUpdate_inflationary

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

end GSet
end Crdt
