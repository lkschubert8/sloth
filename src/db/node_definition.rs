use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum FieldDefinitionType {
    String,
    Int,
    Float,
    Boolean,
}

#[derive(Debug)]
pub struct FieldDefinition {
    pub field: FieldDefinitionType,
    pub name: String,
}

#[derive(Debug)]
pub struct Node {
    id: u64,
    fields: HashMap<String, Field>,
}

#[derive(Debug)]
pub struct Field {
    pub value: FieldValue,
}

#[derive(Debug)]
pub enum FieldValue {
    String(String),
    Int(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug)]
pub struct InsertField {
    pub value: FieldValue,
    pub name: String,
}

#[derive(Debug)]
pub struct NodeDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub nodes: BTreeMap<u64, Node>,
    pub current_id: u64,
}

impl NodeDefinition {
    pub fn add_node(&mut self, values: Vec<InsertField>) -> Result<u64, String> {
        let fields = HashMap::new();

        let current_id = self.current_id;
        let node = Node {
            id: current_id,
            fields: fields,
        };
        self.nodes.insert(node.id, node);
        self.current_id += 1;
        return Ok(current_id);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_remove_node_definition() {
        let mut node_definition = create_node_definition();

        node_definition.add_node(Vec::new());
        
        assert!(
            node_definition.nodes.len() == 1,
            "Failed to insert node"
        );

        assert!(
            node_definition.current_id == 1,
            "Failed to increment current id"
        );
    }

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
