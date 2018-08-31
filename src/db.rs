use std::collections::HashMap;

pub struct Db {
    pub node_definitions: HashMap<String, NodeDefinition>,
    pub relationship_definitions: HashMap<String, RelationshipDefinition>,
    pub nodes: Vec<NodeDefinition>,
}

impl Db {
    fn add_node_definition(&mut self, new_node_definition: NodeDefinition) -> () {
        self.node_definitions.insert(new_node_definition.name ,new_node_definition);
    }

    fn remove_node_definition(&mut self, node_definition_name: String) -> () {
        unimplemented!();
    }

    fn add_relationship_definition(&mut self, new_relationship_definition: RelationshipDefinition) -> () {
        self.relationship_definitions.insert(new_relationship_definition.name, new_relationship_definition);
    }

    fn remove_relationship_definition(&mut self, relationship_defintion_name: String) -> () {
        unimplemented!();
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Directionality {
    OneWay,
    TwoWay,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Connection {
    pub reversible: bool,
    pub left_type: NodeDefinition,
    pub right_type: NodeDefinition,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RelationshipFieldDefinition {
    pub name: String,
    pub field_type: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RelationshipDefinition {
    pub name: String,
    pub directionality: Directionality,
    pub connections: Vec<Connection>,
    pub fields: Vec<RelationshipFieldDefinition>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct NodeFieldDefinition {
    pub name: String,
    pub field_type: String,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct NodeDefinition {
    pub name: String,
    pub fields: Vec<NodeFieldDefinition>,
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
    }

    #[test]
    fn test_remove_node_definition() {
        let new_node_definition = NodeDefinition {
            name: String::from("Test Node Name"),
            fields: Vec::new(),
        };
        let mut node_definitions = HashMap::new();
        node_definitions.insert(new_node_definition, new_node_definition);

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
            node_definitions: Vec::new(),
            relationship_definitions: Vec::new(),
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
    }

    #[test]
    fn test_remove_relationship_definition() {
        let new_relationship_definition = RelationshipDefinition {
            name: String::from("Test Relationship Name"),
            directionality: Directionality::OneWay,
            connections: Vec::new(),
            fields: Vec::new(),
        };
        let mut node_relationships = Vec::new();
        node_relationships.push(new_relationship_definition);

        let mut test_db = Db {
            node_definitions: Vec::new(),
            relationship_definitions: node_relationships,
            nodes: Vec::new(),
        };
        test_db.remove_relationship_definition(String::from("Test Relationship Name"));
        assert!(
            test_db.relationship_definitions.len() == 0,
            "Failed to delete node definition"
        );
    }

}
