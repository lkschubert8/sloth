A graph db (for now)

Relations
    All relationships are two way
    Typed Or Untyped
    Weighted Or Unweighted
    Singular or Multiple in either direction
        Can only have one home but can have multiple houses
        House can have multiple owners, multiple tenants

Relationships can be typed
    e.g. given person type Parent can only be Person<->Person (Two way typed)
         given person and animal Pet Person->Animal (One way typed)
    Relationships can be temporal (Lived here at x time)

Traversal?
Return a set or the spanning tree?
Handling closed loops?

Writing my dsl
Search, create types, insert, patch, delete

Allowed field types?
    INT
    FLOAT
    STRING
    BOOLEAN

Build Hashmaps of type definitions to build from at runtime
use lalrpop or whatever the hell its called to build the parser
    


Thinking junk through

User creates some types (Person, Animal, Place)

User creates some relationships (Friend, Pet, Home )
Reverse any nonreversible with of
    Home
        Person<->Place
        Animal<->Place
    Pet
        Person<->Animal
        
User adds in some people and places
personA
placeA

User links them together
personA<-Home->placeA

User wants to find where 

Object Creation language
--Node and Relationship definitions must be unique
    create node {node} ({fieldname} {fieldtype},...)
    create relationship {relationship} {stuff}
    new {node} {name} -> nodeId
    new {relationship} {nodeId} {nodeId} -> leftNodeId
    new rel Home (new node Person stuff) (new node Place stuff)

Find
    find Person where comparisons -> [nodeId]
    findOne Person Where comparisons -> nodeId

Relations
    relation |depth| {nodeId} {nodeId}


comparisons
    comparable anything that supports the following operations >,>=,<=,<, ==
    regex (string)

Tree generation (have to prevent looping)
    Should it be root to leaf or general acceptable traversals?
    tree [nodeIds] [listOfAcceptablelinkages] -> [Tree]
    e.g. tree [1,2,3] [Person->{Home Comparisons}->Place]

Utility
    union<t> [list<t>] [list<t>] -> [list<t> + list<t>]
    exclude<t> [list<t>] [list<t>] -> [list<t> - list<t>]
    join<t>
    outer<t>
    math ops (),**,*,/,-,+


how do you get a reference to a node? find/findOne
Do they all have identifiers? yes they have a hashId
Make initial searches not the key focus but the graph hops

indices on fields are stored as b trees with pointers to their respective node (and potentially pointers to their location on disc)

How to implement transactions witthout having duplicate? Make a shadow set that unions to the real data? Make's things slower I think?
