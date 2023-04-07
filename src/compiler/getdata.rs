use std::{collections::BTreeMap, fs};

use rhai::{
    serde::{from_dynamic, to_dynamic},
    Dynamic, EvalAltResult, FnPtr, Map, Scope,
};
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use regex::Regex;

use crate::{compiler::Value, types::Datum};

#[derive(Deserialize)]
pub struct Objects {
    #[serde(rename = "type")]
    pub typ: i32,
    pub filename: String,
}

#[derive(Deserialize)]
pub struct Rules {
    #[serde(rename = "type")]
    pub typ: Option<i32>,
    pub description: String,
    pub args: Option<Vec<String>>,
    pub datum: Option<Datum>,
}

#[derive(Deserialize)]
pub struct Blocks {
    #[serde(rename = "type")]
    pub typ: i32,
    pub description: String,
    pub args: Option<Vec<String>>,
    pub datum: Option<Datum>,
}

#[derive(Deserialize)]
pub struct CompiledData {
    pub objects: BTreeMap<String, Objects>,
    pub rules: BTreeMap<String, Rules>,
    pub blocks: BTreeMap<String, Blocks>,
}

struct ParsedString(String);

impl<'de> Deserialize<'de> for ParsedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let res = Regex::new(r"\{arg\d+\}").unwrap();

        let s: &str = Deserialize::deserialize(deserializer)?;

        for v in res.captures_iter(s) {
            println!("{:?}", v);
        };

        todo!()
    }
}

fn uuid() -> Result<String, Box<EvalAltResult>> {
    Ok(Uuid::new_v4().to_string().to_uppercase())
}

fn get_fnptr_list(name: &str, scope: &Scope) -> Vec<FnPtr> {
    scope
        .get(name)
        .unwrap()
        .to_owned()
        .into_typed_array::<FnPtr>()
        .unwrap()
}

fn paramset(value: Dynamic, mut map: Map) -> Result<Map, Box<EvalAltResult>> {
    let val: Value = from_dynamic(&value).unwrap();

    if val.datum.is_some() {
        map.insert("datum".into(), to_dynamic(val.datum).unwrap());
    }

    map.insert("value".into(), to_dynamic(val.value).unwrap());

    Ok(map)
}

pub fn generate_data(path: &str) -> CompiledData {
    let data = fs::read_to_string(path).expect("Error reading data.");
    serde_yaml::from_str(&data).expect("Failed to parse data.")
}
