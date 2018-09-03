use std::fs::File;
use std::io::prelude::*;
use serde_yaml;

use std::collections::HashMap;

pub mod node_definition;
use self::node_definition::NodeDefinition;
use self::node_definition::NodeFieldDefinition;

pub mod field_type;
use self::field_type::FieldType;

//#[derive(Serialize, Deserialize)]
pub struct Db<'a> {
    pub node_definitions: HashMap<&'a str, NodeDefinition>,
    pub relationship_definitions: HashMap<&'a str, RelationshipDefinition<'a>>,
    pub nodes: Vec<&'a NodeDefinition>, //This will be changed to a set of sets
}

impl<'a> Db<'a> {
    fn default () -> Db<'a> {
        return Db {
            node_definitions: HashMap::new(),
            relationship_definitions: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    fn load_schema_definition(filename: &str) -> Db {
        unimplemented!();
        // let mut file = File::open(filename).unwrap();
        // let mut contents = String::new();
        // file.read_to_string(&mut contents);
        // return serde_yaml::from_str(contents.as_str()).unwrap();
    }

    fn add_node_definition(&mut self, new_node_definition: NodeDefinition) -> () {
        self.node_definitions.insert(new_node_definition.name.as_str(), new_node_definition);
    }

    fn remove_node_definition(&mut self, node_definition_name: &'a str) -> () {
        self.node_definitions.remove(&node_definition_name);
    }

    fn add_relationship_definition(&mut self, new_relationship_definition: RelationshipDefinition<'a>) -> () {
        self.relationship_definitions.insert(new_relationship_definition.name.as_str(), new_relationship_definition);
    }

    fn remove_relationship_definition(&mut self, relationship_definition_name: &'a str) -> () {
        self.relationship_definitions.remove(&relationship_definition_name);
    }

    fn save_schema_definition(&mut self, filename: &str) -> () {
        unimplemented!();
        // let mut file = File::create(filename).unwrap();
        // file.write_all(serde_yaml::to_string(&self).unwrap().as_bytes());
    }

    
}

#[derive(Debug)]//#[derive(Serialize, Deserialize)]
pub enum Directionality {
    OneWay,
    TwoWay,
}

#[derive(Debug)]//#[derive(Serialize, Deserialize)]
pub struct Connection<'a> {
    pub reversible: bool,
    pub left_type: &'a NodeDefinition,
    pub right_type: &'a NodeDefinition,
}

#[derive(Debug)]//#[derive(Serialize, Deserialize)]
pub struct RelationshipFieldDefinition {
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Debug)]//#[derive(Serialize, Deserialize)]
pub struct RelationshipDefinition<'a> {
    pub name: String,
    pub directionality: Directionality,
    pub connections: Vec<Connection<'a>>,
    pub fields: Vec<RelationshipFieldDefinition>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_node_definition() {
        let new_node_definition = NodeDefinition {
            name: String::from("Test Node Name"),
            fields: Vec::new(),
        };
        let new_node_definition2 = NodeDefinition {
            name: String::from("Test Node Name"),
            fields: Vec::new(),
        };
        let mut test_db = Db::default();

        test_db.add_node_definition(new_node_definition);
        assert!(
            test_db.node_definitions.len() == 1,
            "Failed to add node definition"
        );

       
        test_db.add_node_definition(new_node_definition2);

        assert!(
            test_db.node_definitions.len() == 1,
            "Created duplicate node definition"
        );

    }

    #[test]
    fn test_remove_node_definition() {
        let new_node_definition = NodeDefinition {
            name: String::from("Test Node Name"),
            fields: Vec::new(),
        };
        let mut node_definitions = HashMap::new();
        node_definitions.insert("Test Node Name", new_node_definition);

        let mut test_db = Db {
            node_definitions: node_definitions,
            relationship_definitions: HashMap::new(),
            nodes: Vec::new(),
        };
        test_db.remove_node_definition("Test Node Name");
        assert!(
            test_db.node_definitions.len() == 0,
            "Failed to delete node definition"
        );
    }

    #[test]
    fn test_add_relationship_defintion(){
        let mut test_db = Db {
            node_definitions: HashMap::new(),
            relationship_definitions: HashMap::new(),
            nodes: Vec::new(),
        };
        let new_relationship_definition = RelationshipDefinition {
            name: String::from("Test Relationship Name"),
            directionality: Directionality::OneWay,
            connections: Vec::new(),
            fields: Vec::new(),
        };
        test_db.add_relationship_definition(new_relationship_definition);
        assert!(
            test_db.relationship_definitions.len() == 1,
            "Failed to add relationship definition"
        );
        let new_relationship_definition2 = RelationshipDefinition {
            name: String::from("Test Relationship Name"),
            directionality: Directionality::OneWay,
            connections: Vec::new(),
            fields: Vec::new(),
        };
        test_db.add_relationship_definition(new_relationship_definition2);
         assert!(
            test_db.relationship_definitions.len() == 1,
            "Created duplicate relationship definition"
        );
    }

    #[test]
    fn test_remove_relationship_definition() {
        let new_relationship_definition = RelationshipDefinition {
            name: String::from("Test Relationship Name"),
            directionality: Directionality::OneWay,
            connections: Vec::new(),
            fields: Vec::new(),
        };
        let mut node_relationships = HashMap::new();
        node_relationships.insert("Test Relationship Name", new_relationship_definition);

        let mut test_db = Db {
            node_definitions: HashMap::new(),
            relationship_definitions: node_relationships,
            nodes: Vec::new(),
        };
        test_db.remove_relationship_definition("Test Relationship Name");
        assert!(
            test_db.relationship_definitions.len() == 0,
            "Failed to delete node definition"
        );
    }

    #[test]
    fn test_save_and_restore_db() {
        
        let mut test_db = Db::default();
        //Adding node definitions
        let new_node_definition = NodeDefinition {
            name: String::from("Test Node Name"),
            fields: Vec::new(),
        };
        let new_node_definition2 = NodeDefinition {
            name: String::from("Second Test Node Type"),
            fields: Vec::new(),
        };
        test_db.add_node_definition(new_node_definition);
        test_db.add_node_definition(new_node_definition2);

        let relation = RelationshipDefinition {
            connections: Vec::new(),
            name: String::from("Friend"),
            directionality: Directionality::OneWay,
            fields: Vec::new(),
        };
        test_db.add_relationship_definition(relation);

        test_db.save_schema_definition("./test-schema-drop.slth");

        let mut new_db = Db::load_schema_definition("./test-schema-drop.slth");
        assert_eq!(test_db.node_definitions.len(), new_db.node_definitions.len(), "Did not properly store or load node definitions");
        assert_eq!(test_db.relationship_definitions.len(), new_db.relationship_definitions.len(), "Did not properly store or load relationship definitions");



    }

}