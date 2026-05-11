---- MODULE GCounter ----
EXTENDS Naturals, FiniteSets

CONSTANTS Replicas, MaxCounter

ASSUME MaxCounter \in Nat

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

StateLeq(left, right) ==
    \A actor \in Replicas : left[actor] <= right[actor]

JoinIdempotent ==
    \A state \in StateType :
        PointwiseMax(state, state) = state

JoinCommutative ==
    \A left \in StateType :
        \A right \in StateType :
            PointwiseMax(left, right) = PointwiseMax(right, left)

JoinAssociative ==
    \A left \in StateType :
        \A middle \in StateType :
            \A right \in StateType :
                PointwiseMax(PointwiseMax(left, middle), right) =
                    PointwiseMax(left, PointwiseMax(middle, right))

MergeInflationary ==
    \A left \in StateType :
        \A right \in StateType :
            /\ StateLeq(left, PointwiseMax(left, right))
            /\ StateLeq(right, PointwiseMax(left, right))

MergeMonotone ==
    \A leftBefore \in StateType :
        \A leftAfter \in StateType :
            \A rightBefore \in StateType :
                \A rightAfter \in StateType :
                    /\ StateLeq(leftBefore, leftAfter)
                    /\ StateLeq(rightBefore, rightAfter)
                    => StateLeq(
                        PointwiseMax(leftBefore, rightBefore),
                        PointwiseMax(leftAfter, rightAfter))

TypeOK ==
    /\ states \in [Replicas -> StateType]
    /\ network \subseteq MessageType

NoPhantomIncrements ==
    TypeOK =>
        \A replica \in Replicas :
            \A actor \in Replicas :
                states[replica][actor] <= states[actor][actor]

MessagePayloadsRespectOwners ==
    TypeOK =>
        \A message \in network :
            \A actor \in Replicas :
                message.payload[actor] <= states[actor][actor]

GCounterCorrectness ==
    /\ TypeOK
    /\ JoinIdempotent
    /\ JoinCommutative
    /\ JoinAssociative
    /\ MergeInflationary
    /\ MergeMonotone
    /\ NoPhantomIncrements
    /\ MessagePayloadsRespectOwners

THEOREM GCounterJoinLaws ==
    /\ JoinIdempotent
    /\ JoinCommutative
    /\ JoinAssociative
    /\ MergeInflationary
    /\ MergeMonotone
PROOF
    <1>1. JoinIdempotent
        BY DEF JoinIdempotent, PointwiseMax, Max, StateType
    <1>2. JoinCommutative
        BY DEF JoinCommutative, PointwiseMax, Max, StateType
    <1>3. JoinAssociative
        BY DEF JoinAssociative, PointwiseMax, Max, StateType
    <1>4. MergeInflationary
        BY DEF MergeInflationary, StateLeq, PointwiseMax, Max, StateType
    <1>5. MergeMonotone
        BY DEF MergeMonotone, StateLeq, PointwiseMax, Max, StateType
    <1> QED
        BY <1>1, <1>2, <1>3, <1>4, <1>5

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

StateComponentsDoNotDecrease ==
    \A replica \in Replicas :
        StateLeq(states[replica], states'[replica])

StateMonotonic ==
    [][StateComponentsDoNotDecrease]_<<states, network>>

====
