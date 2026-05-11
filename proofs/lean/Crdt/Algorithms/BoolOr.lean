import Crdt.StateBased.CvRDT

namespace Crdt
namespace BoolOr

abbrev State := Bool

def join (left right : State) : State :=
  left || right

instance : JoinSemilattice State where
  join := join
  join_assoc := by
    intro a b c
    exact Bool.or_assoc a b c
  join_comm := by
    intro a b
    exact Bool.or_comm a b
  join_idem := by
    intro a
    exact Bool.or_self a

def bottom : State :=
  false

def enable (state : State) : State :=
  JoinSemilattice.join state true

theorem enable_inflationary (state : State) :
    leq state (enable state) := by
  unfold enable
  exact leq_join_left state true

theorem merge_converges_for_same_states (left right : State) :
    JoinSemilattice.join left right = JoinSemilattice.join right left :=
  JoinSemilattice.join_comm left right

end BoolOr
end Crdt
