use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Project {
    pub variables: Vec<Variable>,
    pub uuid: String,
    pub objects: Vec<Object>,
    pub rules: Vec<Rule>,
    pub abilities: Vec<Ability>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub typ: i32,
    pub object_id_string: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Object {
    pub name: String,
    pub typ: i32,
    pub filename: String,
    pub id: String,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub rule_block_type: i32,
    pub object_id: String,
    pub id: String,
    pub ability_id: String,
    pub params: Vec<Param>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Param {
    pub value: String,
    pub typ: i32,
    pub default_value: String,
    pub key: String,
    pub datum: Option<Datum>,
    pub variable: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Datum {
    pub typ: i32,
    pub block_class: Option<String>,
    pub params: Option<Vec<Param>>,
    pub variable: Option<String>,
}

#[derive(Debug)]
pub struct Ability {
    pub ability_id: String,
    pub blocks: Vec<Block>,
    pub created_at: i32,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub block_class: String,
    pub typ: i32,
    pub description: String,
    pub parameters: Option<Vec<Param>>,
}
