# Edge

Edges in Surreal represent relationships between Nodes. They are useful when you
want to model many-to-many relationships or when you want to store additional
information about the relationship itself. Edges can be seen as "relationship
tables" in a relational database context, holding metadata about the
relationship between two entities. Edges are defined by a Rust struct that
implements the `Edge` trait.

Here's a detailed example:

```rust, ignore
#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    pub name: String,

    // This is a read-only field
    #[orm(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]
    #[serde(skip_serializing, default)]
    pub planets_to_visit: Relate<Planet>,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = "planet")]
pub struct Planet {
    pub id: SurrealSimpleId<Self>,
    pub population: u64,
}

// Visits
#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = "visits")]
pub struct Visits<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    pub in_: LinkOne<In>,
    pub out: LinkOne<Out>,
    pub time_visited: Duration,
}

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanet = Visits<Alien, Planet>;
```

The `Alien` Node has a field `planets_to_visit` which is of type
`Relate<Planet>`. This field doesn't represent a direct link from `Alien` to
`Planet`. Instead, it represents an indirect relationship via the `Visits` Edge.
This indirect relationship is defined by the `Relate` annotation on the
`planets_to_visit` field in the `Alien` Node.

The
`#[orm(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]`
attribute on the `planets_to_visit` field in the `Alien` Node tells Surreal that
this field represents the `Planet` Nodes that are connected to the `Alien` Node
via the `AlienVisitsPlanet` Edge. The `connection = "->visits->planet"` part
defines the path of the relationship from the `Alien` Node, through the `Visits`
Edge (represented by "visits"), and finally to the `Planet` Node.

The `Visits` Edge struct defines the structure of this relationship. It
implements `Edge` and specifies two type parameters: `In` and `Out` which
represent the source and target Node types of the relationship, respectively. In
this example, `Alien` is the source and `Planet` is the target. The `Visits`
Edge also has a `time_visited` field, which can store additional information
about each visit.

In summary, Surreal Edges provide a flexible way to model complex relationships
between Nodes, such as when an `Alien` visits a `Planet`. They allow for
relationships to be modeled with additional information (like the `time_visited`
field in the `Visits` Edge) and can represent both direct and indirect
connections between Nodes.
