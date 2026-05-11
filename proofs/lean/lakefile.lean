import Lake
open Lake DSL

package crdt

@[default_target]
lean_lib Crdt where
  roots := #[`Crdt]
