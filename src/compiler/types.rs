#[allow(dead_code)]
use serde::Deserialize;

pub struct Fncall {
    pub fnname: String,
}

#[derive(Debug)]
pub struct Project {
    pub variables: Vec<Variable>,
    pub uuid: String,
    pub objects: Vec<Object>,
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
}
