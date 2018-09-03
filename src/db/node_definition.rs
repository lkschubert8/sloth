use super::FieldType;

#[derive(Debug)]//#[derive(Serialize, Deserialize)]
pub struct NodeFieldDefinition {
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Debug)]//#[derive(Serialize, Deserialize)]
pub struct NodeDefinition {
    pub name: String,
    pub fields: Vec<NodeFieldDefinition>,
}