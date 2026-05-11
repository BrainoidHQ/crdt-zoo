namespace Crdt

class JoinSemilattice (α : Type u) where
  join : α -> α -> α
  join_assoc : forall a b c : α, join (join a b) c = join a (join b c)
  join_comm : forall a b : α, join a b = join b a
  join_idem : forall a : α, join a a = a

def leq [JoinSemilattice α] (a b : α) : Prop :=
  JoinSemilattice.join a b = b

theorem leq_refl [JoinSemilattice α] (a : α) : leq a a := by
  unfold leq
  exact JoinSemilattice.join_idem a

theorem leq_join_left [JoinSemilattice α] (a b : α) :
    leq a (JoinSemilattice.join a b) := by
  unfold leq
  rw [← JoinSemilattice.join_assoc]
  rw [JoinSemilattice.join_idem]

theorem leq_join_right [JoinSemilattice α] (a b : α) :
    leq b (JoinSemilattice.join a b) := by
  rw [JoinSemilattice.join_comm a b]
  exact leq_join_left b a

end Crdt
