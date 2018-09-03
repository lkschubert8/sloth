use super::FieldType;
use super::NodeDefinition;

#[derive(Debug)] //#[derive(Serialize, Deserialize)]
pub struct RelationshipDefinition<'a> {
    pub name: String,
    pub directionality: Directionality,
    pub connections: Vec<Connection<'a>>,
    pub fields: Vec<RelationshipFieldDefinition>,
}
#[derive(Debug)] //#[derive(Serialize, Deserialize)]
pub enum Directionality {
    OneWay,
    TwoWay,
}

#[derive(Debug)] //#[derive(Serialize, Deserialize)]
pub struct Connection<'a> {
    pub reversible: bool,
    pub left_type: &'a NodeDefinition,
    pub right_type: &'a NodeDefinition,
}

#[derive(Debug)] //#[derive(Serialize, Deserialize)]
pub struct RelationshipFieldDefinition {
    pub name: String,
    pub field_type: FieldType,
}
