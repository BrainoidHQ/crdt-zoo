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

theorem merge_converges_for_same_states (left right : State Element) :
    JoinSemilattice.join left right = JoinSemilattice.join right left :=
  JoinSemilattice.join_comm left right

end GSet
end Crdt
