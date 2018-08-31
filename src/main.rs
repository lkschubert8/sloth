#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;


pub mod db;
use std::collections::HashMap;

fn main() {
    let relationships: HashMap<String, db::RelationshipDefinition> = HashMap::new();
    let nodes: HashMap<String, db::NodeDefinition> = HashMap::new();
    let new_type = db::NodeDefinition {
        name: String::from("Person"),
        fields: Vec::new(),
    };

    let new_type_2 = db::NodeDefinition {
        name: String::from("Person"),
        fields: Vec::new(),
    };

    let p2p_connection = db::Connection {
        reversible: true,
        left_type: new_type,
        right_type: new_type_2,
    };

    let relation = db::RelationshipDefinition {
        connections: vec![p2p_connection],
        name: String::from("Friend"),
        directionality: db::Directionality::OneWay,
        fields: Vec::new(),
    };

    let t = db::NodeFieldDefinition {
        name: String::from("Test"),
        field_type: db::FieldType::String
    };
}
