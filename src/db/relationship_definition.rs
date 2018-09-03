use super::FieldType;
use super::NodeDefinition;

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationshipDefinition {
    pub name: String,
    pub directionality: Directionality,
    pub connections: Vec<Connection>,
    pub fields: Vec<RelationshipFieldDefinition>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Directionality {
    OneWay,
    TwoWay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub reversible: bool,
    pub left_type: String,
    pub right_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationshipFieldDefinition {
    pub name: String,
    pub field_type: FieldType,
}

pub struct Relationship {
    id: u64,
}
