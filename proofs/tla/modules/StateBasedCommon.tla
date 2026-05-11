---- MODULE StateBasedCommon ----
EXTENDS Naturals

Max(a, b) ==
    IF a >= b THEN a ELSE b

PointwiseMax(left, right) ==
    [actor \in DOMAIN left \cup DOMAIN right |->
        IF actor \in DOMAIN left /\ actor \in DOMAIN right THEN
            Max(left[actor], right[actor])
        ELSE IF actor \in DOMAIN left THEN
            left[actor]
        ELSE
            right[actor]]

StateMessage(ReplicaSet, StateSet) ==
    [from: ReplicaSet, to: ReplicaSet, payload: StateSet]

NetworkType(ReplicaSet, StateSet) ==
    SUBSET StateMessage(ReplicaSet, StateSet)

StateMessageRecord(src, dst, state) ==
    [from |-> src, to |-> dst, payload |-> state]

EnqueueState(network, states, src, dst) ==
    network \cup {StateMessageRecord(src, dst, states[src])}

DequeueMessage(network, message) ==
    network \ {message}

====
