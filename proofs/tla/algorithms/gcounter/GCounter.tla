---- MODULE GCounter ----
EXTENDS Naturals, FiniteSets

CONSTANTS Replicas, MaxCounter

VARIABLES states, network

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

Zero ==
    [actor \in Replicas |-> 0]

StateType ==
    [Replicas -> 0..MaxCounter]

MessageType ==
    [from: Replicas, to: Replicas, payload: StateType]

TypeOK ==
    /\ states \in [Replicas -> StateType]
    /\ network \subseteq MessageType

Init ==
    /\ states = [replica \in Replicas |-> Zero]
    /\ network = {}

Inc(replica) ==
    /\ states[replica][replica] < MaxCounter
    /\ states' = [states EXCEPT ![replica][replica] = @ + 1]
    /\ UNCHANGED network

SendState(src, dst) ==
    /\ src # dst
    /\ network' = network \cup {[from |-> src, to |-> dst, payload |-> states[src]]}
    /\ UNCHANGED states

Deliver(message) ==
    /\ message \in network
    /\ states' = [states EXCEPT ![message.to] = PointwiseMax(@, message.payload)]
    /\ network' = network \ {message}

Next ==
    \/ \E replica \in Replicas : Inc(replica)
    \/ \E src \in Replicas :
        \E dst \in Replicas : SendState(src, dst)
    \/ \E message \in network : Deliver(message)

Spec ==
    Init /\ [][Next]_<<states, network>>

====
