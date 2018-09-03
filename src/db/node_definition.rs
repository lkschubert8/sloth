use std::collections::BTreeMap;

use super::FieldType;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeFieldDefinition {
    pub name: String,
    pub field_type: FieldType,
    pub index: bool,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDefinition {
    pub name: String,
    pub fields: Vec<NodeFieldDefinition>,
}

pub struct Node {
    pub id: u64,
    pub fields: Vec<String>,
    pub relationships: Vec<String>,
}

pub struct NodeGroup {
    pub definition: NodeDefinition,
    pub current_id: u64,
    pub nodes: BTreeMap<u64, Node>,
}