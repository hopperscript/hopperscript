use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Project {
    pub variables: Vec<Variable>,
    pub uuid: String,
    pub objects: Vec<Object>,
    pub rules: Vec<Rule>,
    pub abilities: Vec<Ability>,
    pub scenes: Vec<Scene>,
}

#[derive(Debug)]
pub struct Scene {
    pub name: String,
    pub objects: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: i32,
    #[serde(rename = "objectIdString")]
    pub object_id_string: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct Object {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: i32,
    pub filename: String,
    pub id: String,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Rule {
    #[serde(rename = "ruleBlockType")]
    pub rule_block_type: i32,
    #[serde(rename = "objectID")]
    pub object_id: String,
    pub id: String,
    #[serde(rename = "abilityID")]
    pub ability_id: String,
    #[serde(rename = "parameters")]
    pub params: Vec<Param>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Param {
    pub value: String,
    #[serde(rename = "type")]
    pub typ: i32,
    #[serde(rename = "defaultValue")]
    pub default_value: String,
    pub key: String,
    pub datum: Option<Datum>,
    pub variable: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Datum {
    #[serde(rename = "type")]
    pub typ: i32,
    pub block_class: Option<String>, // not camelcase
    pub params: Option<Vec<Param>>,  // keep 'params'
    pub variable: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Ability {
    #[serde(rename = "abilityID")]
    pub ability_id: String,
    pub blocks: Vec<Block>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    pub block_class: String, // not camelcase
    #[serde(rename = "type")]
    pub typ: i32,
    pub description: String,
    pub parameters: Option<Vec<Param>>,
}
