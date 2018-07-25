pub mod db;
use std::collections::HashMap;
use db::relationships;

fn main() {
    let relationships: HashMap<String, relationships::RelationshipDefinition> = HashMap::new();
    let nodes: HashMap<String, relationships::NodeDefinition> = HashMap::new();
    let new_type = relationships::NodeDefinition {
        name: String::from("Person"),
        fields: Vec::new(),
    };

    let new_type_2 = relationships::NodeDefinition {
        name: String::from("Person"),
        fields: Vec::new(),
    };

    let p2p_connection = relationships::Connection {
        reversible: true,
        left_type: new_type,
        right_type: new_type_2,
    };

    let relation = relationships::RelationshipDefinition {
        connections: vec![p2p_connection],
        name: String::from("Friend"),
        directionality: relationships::Directionality::OneWay,
        fields: Vec::new(),
    };

    println!("{:?}", relation);
}
