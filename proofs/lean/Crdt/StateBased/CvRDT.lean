import Crdt.Algebra.JoinSemilattice

namespace Crdt

class CvRDT (State : Type u) [JoinSemilattice State] where
  Update : Type v
  Query : Type w
  apply : Update -> State -> State
  query : State -> Query
  apply_inflationary : forall update state, leq state (apply update state)

def merge [JoinSemilattice State] (left right : State) : State :=
  JoinSemilattice.join left right

theorem merge_comm [JoinSemilattice State] (left right : State) :
    merge left right = merge right left :=
  JoinSemilattice.join_comm left right

theorem merge_assoc [JoinSemilattice State] (a b c : State) :
    merge (merge a b) c = merge a (merge b c) :=
  JoinSemilattice.join_assoc a b c

theorem merge_idem [JoinSemilattice State] (state : State) :
    merge state state = state :=
  JoinSemilattice.join_idem state

theorem converge_when_mutually_observed [JoinSemilattice State] {left right : State} :
    leq left right -> leq right left -> left = right :=
  leq_antisymm

theorem converge_with_same_observed_join [JoinSemilattice State]
    {left right observed : State} :
    leq left observed ->
    leq observed left ->
    leq right observed ->
    leq observed right ->
    left = right := by
  intro leftBelow observedBelowLeft rightBelow observedBelowRight
  have leftEqObserved : left = observed :=
    leq_antisymm leftBelow observedBelowLeft
  have rightEqObserved : right = observed :=
    leq_antisymm rightBelow observedBelowRight
  exact leftEqObserved.trans rightEqObserved.symm

end Crdt
