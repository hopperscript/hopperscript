use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Project {
    pub variables: Vec<Variable>,
    pub uuid: String,
    pub objects: Vec<Object>,
    pub rules: Vec<Rule>,
    pub abilities: Vec<Ability>,
    pub scenes: Vec<Scene>,
    #[serde(rename = "eventParameters")]
    pub event_params: Vec<EventParam>,
}

#[derive(Debug, Serialize, Clone)]
pub struct EventParam {
    pub id: String,
    pub description: String,
    #[serde(rename = "blockType")]
    pub block_type: i32,
    #[serde(rename = "objectID")]
    pub object_id: Option<String>,
}

#[derive(Debug, Serialize)]
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
    #[serde(rename = "objectID")]
    pub id: String,
    pub rules: Vec<String>,
    #[serde(rename = "xPosition")]
    pub x: i32,
    #[serde(rename = "yPosition")]
    pub y: i32,
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
    pub object: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Ability {
    #[serde(rename = "abilityID")]
    pub ability_id: String,
    pub blocks: Vec<Block>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ControlScript {
    #[serde(rename = "abilityID")]
    pub ability_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Block {
    pub block_class: String, // not camelcase
    #[serde(rename = "type")]
    pub typ: i32,
    pub description: String,
    pub parameters: Option<Vec<Param>>,
    #[serde(rename = "controlScript")]
    pub control_script: Option<ControlScript>,
}

// TYPES
#[derive(Clone, Debug)]
pub enum DefineTypes {
    Object(String),
    /// i32 = the "code"
    Variable(i32),
    Ability(Option<Vec<BlockAST>>),
}

#[derive(Debug, Clone)]
pub struct BlockAST {
    pub name: String,
    pub params: Vec<Values>,
    pub typ: AstTypes,
}

#[derive(Clone, Debug)]
pub enum Values {
    Object(String),
    Str(String),
    Variable(String, i32),
    ObjectVariable(String, String),
    Conditional(Box<Values>, String, Box<Values>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstTypes {
    Block,
    Ability,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub value: String,
    pub datum: Option<Datum>,
}

#[derive(Clone, Debug)]
pub enum Script {
    Define {
        typ: DefineTypes,
        name: String,
    },
    //Loop(Vec<Self>),
    On {
        obj: String,
        con: Vec<Script>,
    },
    Rule {
        name: String,
        con: Vec<BlockAST>,
        params: Vec<Values>,
    },
}

pub struct CompiledData {
    pub obj: Vec<ObjectData>,
    pub rules: Vec<BlockData>,
    pub blocks: Vec<BlockData>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct BlockData {
    pub name: String,
    pub parameters: Vec<ParameterData>,
    pub id: i32,
    #[serde(rename = "type")]
    pub typ: String,
    pub label: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ParameterData {
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ObjectData {
    #[serde(default)]
    pub name: String,
    pub id: i32,
}