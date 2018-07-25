pub struct Db {
    pub node_definitons: Vec<NodeDefinition>,
    pub relationship_definitions : Vec<RelationshipDefinition>,
}

impl Db {
    fn add_node(&mut self, newNode : NodeDefinition) -> () {
        self.node_definitons.push(newNode)
    }
}


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
pub struct RelationshipFieldDefinition {
    pub name: String,
    pub field_type: String,
}


#[derive(Debug)]
pub struct RelationshipDefinition {
    pub name: String,
    pub directionality: Directionality,
    pub connections: Vec<Connection>,
    pub fields: Vec<RelationshipFieldDefinition>
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
