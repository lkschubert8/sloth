use serde_yaml;
use std::fs::File;
use std::io::prelude::*;

use std::collections::BTreeMap;
use std::collections::HashMap;

pub mod node_definition;
use self::node_definition::*;

#[derive(Debug)]
pub struct Db {
    pub node_set: HashMap<String, NodeDefinition>,
}

impl Db {
    fn default() -> Db {
        return Db {
            node_set: HashMap::new(),
        };
    }

    fn load(filename: &str) -> Db {
        unimplemented!();
    }

    fn add_node_definition(&mut self, new_node_definition: NodeDefinition) -> () {
        self.node_set.insert(new_node_definition.name.clone(), new_node_definition);
    }

    fn remove_node_definition(&mut self, node_definition_name: String) -> () {
        self.node_set.remove_entry(node_definition_name.as_str());
    }

    // fn add_relationship_definition(
    //     &mut self,
    //     new_relationship_definition: RelationshipDefinition,
    // ) -> () {
    //   unimplemented!();
    // }

    // fn remove_relationship_definition(&mut self, relationship_definition_name: String) -> () {
    //     self.relationship_definitions
    //         .remove(&relationship_definition_name);
    // }

    fn save(&mut self, filename: &str) -> () {
        unimplemented!();
    }

    fn add_node(&mut self, node_type: String, new_node: Node) -> u64 {
        unimplemented!();
    }

    fn remove_node(&mut self, node_id: u64) -> () {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_remove_node_definition() {
        let new_node_definition = create_node_definition();

        let mut db = Db::default();

        db.add_node_definition(new_node_definition);

        assert!(
            db.node_set.len() == 1,
            "Failed to insert node definition"
        );

        db.remove_node_definition(String::from("Person"));

        assert!(
            db.node_set.len() == 0,
            "Failed to remove node definition"
        )
    }

    // #[test]
    // fn test_add_relationship_defintion() {
    //     let mut test_db = Db {
    //         node_groups: HashMap::new(),
    //         relationship_definitions: HashMap::new(),
    //     };
    //     let new_relationship_definition = RelationshipDefinition {
    //         name: String::from("Test Relationship Name"),
    //         directionality: Directionality::OneWay,
    //         connections: Vec::new(),
    //         fields: Vec::new(),
    //     };
    //     test_db.add_relationship_definition(new_relationship_definition);
    //     assert!(
    //         test_db.relationship_definitions.len() == 1,
    //         "Failed to add relationship definition"
    //     );
    //     let new_relationship_definition2 = RelationshipDefinition {
    //         name: String::from("Test Relationship Name"),
    //         directionality: Directionality::OneWay,
    //         connections: Vec::new(),
    //         fields: Vec::new(),
    //     };
    //     test_db.add_relationship_definition(new_relationship_definition2);
    //     assert!(
    //         test_db.relationship_definitions.len() == 1,
    //         "Created duplicate relationship definition"
    //     );
    // }

    // #[test]
    // fn test_remove_relationship_definition() {
    //     let new_relationship_definition = RelationshipDefinition {
    //         name: String::from("Test Relationship Name"),
    //         directionality: Directionality::OneWay,
    //         connections: Vec::new(),
    //         fields: Vec::new(),
    //     };
    //     let mut node_relationships = HashMap::new();
    //     node_relationships.insert(
    //         String::from("Test Relationship Name"),
    //         new_relationship_definition,
    //     );

    //     let mut test_db = Db {
    //         node_groups: HashMap::new(),
    //         relationship_definitions: node_relationships,
    //     };
    //     test_db.remove_relationship_definition(String::from("Test Relationship Name"));
    //     assert!(
    //         test_db.relationship_definitions.len() == 0,
    //         "Failed to delete node definition"
    //     );
    // }

    // #[test]
    // fn test_save_and_restore_db() {
    //     let mut test_db = Db::default();
    //     //Adding node definitions
    //     let new_node_definition = NodeDefinition {
    //         name: String::from("Test Node Name"),
    //         fields: Vec::new(),
    //     };
    //     let new_node_definition2 = NodeDefinition {
    //         name: String::from("Second Test Node Type"),
    //         fields: Vec::new(),
    //     };
    //     test_db.add_node_definition(new_node_definition);
    //     test_db.add_node_definition(new_node_definition2);

    //     let relation = RelationshipDefinition {
    //         connections: Vec::new(),
    //         name: String::from("Friend"),
    //         directionality: Directionality::OneWay,
    //         fields: Vec::new(),
    //     };
    //     test_db.add_relationship_definition(relation);

    //     test_db.save("./test-schema-drop.slth");

    //     let mut new_db = Db::load("./test-schema-drop.slth");
    //     assert_eq!(
    //         test_db.node_groups.len(),
    //         new_db.node_groups.len(),
    //         "Did not properly store or load node definitions"
    //     );
    //     assert_eq!(
    //         test_db.relationship_definitions.len(),
    //         new_db.relationship_definitions.len(),
    //         "Did not properly store or load relationship definitions"
    //     );
    // }

    //Helper functions
    fn create_node_definition() -> NodeDefinition {
        let field = FieldDefinition {
            field: FieldDefinitionType::String,
            name: String::from("Name"),
        };

        let name = String::from("Person");
        NodeDefinition {
            name: name,
            fields: vec![field],
            nodes: BTreeMap::new(),
            current_id: 0,
        }
    }

}
