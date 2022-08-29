use serde::Deserialize;

#[derive(Debug)]
pub struct Project {
    pub variables: Vec<Variable>,
    pub uuid: String,
    pub objects: Vec<Object>,
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub typ: i32,
    pub object_id_string: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Object {
    pub name: String,
    pub typ: i32,
    pub filename: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub rule_block_type: i32,
    pub object_id: String,
    pub id: String,
    pub ability_id: String,
    pub parameters: Vec<Param>,
}

#[derive(Debug, Deserialize)]
pub struct Param {
    pub value: String,
    pub typ: i32,
    pub default_value: String,
    pub key: String,
    pub datum: Datum,
}

#[derive(Debug, Deserialize)]
pub struct Datum {
    pub typ: i32,
    pub block_class: String,
}
