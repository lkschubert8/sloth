#[derive(Debug)]
pub enum Directionality {
    OneWay,
    TwoWay
}

#[derive(Debug)]
pub struct Connection {
    pub reversible: bool,
    pub left_type: NodeDefinition,
    pub right_type: NodeDefinition
}


#[derive(Debug)]
pub struct RelationshipDefinition {
    pub name: String,
    pub directionality: Directionality,
    pub connections: Vec<Connection>
}

#[derive(Debug)]
pub struct NodeFieldDefinition {
    pub name: String,
    pub field_type: String
}

#[derive(Debug)]
pub struct NodeDefinition {
    pub name: String,
    pub fields: Vec<NodeFieldDefinition>
}

