use std::fs::File;
use std::io::prelude::*;
use serde_yaml;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Db {
    pub node_definitions: HashMap<String, NodeDefinition>,
    pub relationship_definitions: HashMap<String, RelationshipDefinition>,
    pub nodes: Vec<NodeDefinition>,
}

impl Db {
    fn default () -> Db {
        return Db {
            node_definitions: HashMap::new(),
            relationship_definitions: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    fn load_schema_definition(filename: String) -> Db {
        let mut file = File::open(filename.as_str()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        return serde_yaml::from_str(contents.as_str()).unwrap();
    }

    fn add_node_definition(&mut self, new_node_definition: NodeDefinition) -> () {
        self.node_definitions.insert(new_node_definition.name.clone(), new_node_definition);
    }

    fn remove_node_definition(&mut self, node_definition_name: String) -> () {
        self.node_definitions.remove(&node_definition_name);
    }

    fn add_relationship_definition(&mut self, new_relationship_definition: RelationshipDefinition) -> () {
        self.relationship_definitions.insert(new_relationship_definition.name.clone(), new_relationship_definition);
    }

    fn remove_relationship_definition(&mut self, relationship_definition_name: String) -> () {
        self.relationship_definitions.remove(&relationship_definition_name);
    }

    fn save_schema_definition(&mut self, filename: String) -> () {
        let mut file = File::create(filename.as_str()).unwrap();
        file.write_all(serde_yaml::to_string(&self).unwrap().as_bytes());
    }

    
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Directionality {
    OneWay,
    TwoWay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection<'a> {
    pub reversible: bool,
    pub left_type: &'a NodeDefinition,
    pub right_type: &'a NodeDefinition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationshipFieldDefinition {
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationshipDefinition {
    pub name: String,
    pub directionality: Directionality,
    pub connections: Vec<&Connection>,
    pub fields: Vec<RelationshipFieldDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeFieldDefinition {
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDefinition {
    pub name: String,
    pub fields: Vec<NodeFieldDefinition>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_node_definition() {
        let mut test_db = Db {
            node_definitions: HashMap::new(),
            relationship_definitions: HashMap::new(),
            nodes: Vec::new(),
        };
        let new_node_definition = NodeDefinition {
            name: String::from("Test Node Name"),
            fields: Vec::new(),
        };
        test_db.add_node_definition(new_node_definition);
        assert!(
            test_db.node_definitions.len() == 1,
            "Failed to add node definition"
        );

        let new_node_definition2 = NodeDefinition {
            name: String::from("Test Node Name"),
            fields: Vec::new(),
        };
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
        node_definitions.insert(String::from("Test Node Name"), new_node_definition);

        let mut test_db = Db {
            node_definitions: node_definitions,
            relationship_definitions: HashMap::new(),
            nodes: Vec::new(),
        };
        test_db.remove_node_definition(String::from("Test Node Name"));
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
        node_relationships.insert(String::from("Test Relationship Name"), new_relationship_definition);

        let mut test_db = Db {
            node_definitions: HashMap::new(),
            relationship_definitions: node_relationships,
            nodes: Vec::new(),
        };
        test_db.remove_relationship_definition(String::from("Test Relationship Name"));
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

        //Adding relationship definitions
        let p2p_connection = Connection {
            reversible: true,
            left_type: &new_node_definition,
            right_type: &new_node_definition2,
        };

        let relation = RelationshipDefinition {
            connections: vec![p2p_connection],
            name: String::from("Friend"),
            directionality: Directionality::OneWay,
            fields: Vec::new(),
        };
        test_db.add_relationship_definition(relation);

        test_db.save_schema_definition(String::from("./test-schema-drop.slth"));

        let mut new_db = Db::load_schema_definition(String::from("./test-schema-drop.slth"));
        assert_eq!(test_db.node_definitions.len(), new_db.node_definitions.len(), "Did not properly store or load node definitions");
        assert_eq!(test_db.relationship_definitions.len(), new_db.relationship_definitions.len(), "Did not properly store or load relationship definitions");



    }

}
