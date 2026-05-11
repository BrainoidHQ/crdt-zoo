---- MODULE GCounter ----
EXTENDS Naturals, FiniteSets

CONSTANTS Replicas, MaxCounter

ASSUME MaxCounterIsNat == MaxCounter \in Nat

VARIABLES states, network

Vars ==
    <<states, network>>

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

OwnerState ==
    [actor \in Replicas |-> states[actor][actor]]

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
            StateLeq(states[replica], OwnerState)

MessagePayloadsRespectOwners ==
    TypeOK =>
        \A message \in network :
            StateLeq(message.payload, OwnerState)

GCounterCorrectness ==
    /\ TypeOK
    /\ JoinIdempotent
    /\ JoinCommutative
    /\ JoinAssociative
    /\ MergeInflationary
    /\ MergeMonotone
    /\ NoPhantomIncrements
    /\ MessagePayloadsRespectOwners

DistributedInvariant ==
    /\ TypeOK
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

THEOREM PointwiseMaxType ==
    \A left \in StateType :
        \A right \in StateType :
            PointwiseMax(left, right) \in StateType
PROOF
    BY MaxCounterIsNat DEF PointwiseMax, Max, StateType

THEOREM PointwiseMaxUpperBound ==
    \A left \in StateType :
        \A right \in StateType :
            \A upper \in StateType :
                /\ StateLeq(left, upper)
                /\ StateLeq(right, upper)
                => StateLeq(PointwiseMax(left, right), upper)
PROOF
    BY DEF StateLeq, PointwiseMax, Max, StateType

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
    Init /\ [][Next]_Vars

THEOREM GCounterDistributedInvariantInductive ==
    /\ Init => DistributedInvariant
    /\ DistributedInvariant /\ [Next]_Vars => DistributedInvariant'
PROOF
    <1>1. Init => DistributedInvariant
        BY MaxCounterIsNat DEF Init, DistributedInvariant, TypeOK, NoPhantomIncrements,
            MessagePayloadsRespectOwners, OwnerState, StateLeq, Zero, StateType,
            MessageType
    <1>2. DistributedInvariant /\ UNCHANGED Vars => DistributedInvariant'
        BY DEF DistributedInvariant, TypeOK, NoPhantomIncrements,
            MessagePayloadsRespectOwners, OwnerState, StateLeq, Vars
    <1>3. \A replica \in Replicas :
            DistributedInvariant /\ Inc(replica) => DistributedInvariant'
        BY MaxCounterIsNat
            DEF DistributedInvariant, TypeOK, NoPhantomIncrements,
                MessagePayloadsRespectOwners, OwnerState, StateLeq, Inc,
                StateType, MessageType
    <1>4. \A src \in Replicas :
            \A dst \in Replicas :
                DistributedInvariant /\ SendState(src, dst) => DistributedInvariant'
        BY DEF DistributedInvariant, TypeOK, NoPhantomIncrements,
            MessagePayloadsRespectOwners, OwnerState, StateLeq, SendState,
            StateType, MessageType
    <1>5. \A message \in network :
            DistributedInvariant /\ Deliver(message) => DistributedInvariant'
        BY PointwiseMaxType, PointwiseMaxUpperBound
            DEF DistributedInvariant, TypeOK, NoPhantomIncrements,
                MessagePayloadsRespectOwners, OwnerState, StateLeq, Deliver,
                PointwiseMax, Max, StateType, MessageType
    <1>6. DistributedInvariant /\ [Next]_Vars => DistributedInvariant'
        BY <1>2, <1>3, <1>4, <1>5 DEF Next, Vars
    <1> QED
        BY <1>1, <1>6

StateComponentsDoNotDecrease ==
    \A replica \in Replicas :
        StateLeq(states[replica], states'[replica])

StateMonotonic ==
    [][StateComponentsDoNotDecrease]_Vars

====
