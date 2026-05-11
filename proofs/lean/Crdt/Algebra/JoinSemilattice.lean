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

theorem leq_trans [JoinSemilattice α] {a b c : α} :
    leq a b -> leq b c -> leq a c := by
  intro hab hbc
  unfold leq at *
  calc
    JoinSemilattice.join a c = JoinSemilattice.join a (JoinSemilattice.join b c) := by
      rw [hbc]
    _ = JoinSemilattice.join (JoinSemilattice.join a b) c := by
      rw [JoinSemilattice.join_assoc]
    _ = JoinSemilattice.join b c := by
      rw [hab]
    _ = c := hbc

theorem leq_antisymm [JoinSemilattice α] {a b : α} :
    leq a b -> leq b a -> a = b := by
  intro hab hba
  unfold leq at *
  calc
    a = JoinSemilattice.join b a := hba.symm
    _ = JoinSemilattice.join a b := JoinSemilattice.join_comm b a
    _ = b := hab

theorem leq_join_left [JoinSemilattice α] (a b : α) :
    leq a (JoinSemilattice.join a b) := by
  unfold leq
  rw [← JoinSemilattice.join_assoc]
  rw [JoinSemilattice.join_idem]

theorem leq_join_right [JoinSemilattice α] (a b : α) :
    leq b (JoinSemilattice.join a b) := by
  rw [JoinSemilattice.join_comm a b]
  exact leq_join_left b a

theorem join_leq [JoinSemilattice α] {a b c : α} :
    leq a c -> leq b c -> leq (JoinSemilattice.join a b) c := by
  intro hac hbc
  unfold leq at *
  calc
    JoinSemilattice.join (JoinSemilattice.join a b) c
        = JoinSemilattice.join a (JoinSemilattice.join b c) := by
      rw [JoinSemilattice.join_assoc]
    _ = JoinSemilattice.join a c := by
      rw [hbc]
    _ = c := hac

theorem join_monotone [JoinSemilattice α] {a b c d : α} :
    leq a b -> leq c d ->
    leq (JoinSemilattice.join a c) (JoinSemilattice.join b d) := by
  intro hab hcd
  apply join_leq
  · exact leq_trans hab (leq_join_left b d)
  · exact leq_trans hcd (leq_join_right b d)

end Crdt
