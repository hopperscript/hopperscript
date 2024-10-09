use std::{fs, path::Path};

use chumsky::primitive::Container;
use rhai::{
    serde::{from_dynamic, to_dynamic},
    Array, Dynamic, Engine, EvalAltResult, FnPtr, Map, Scope, AST,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{compiler::Value, types::Block};

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
    // TODO: SUPPORT CUSTOM PATHS
    let contents = include_str!("newdata.json");
    let objcontents = include_str!("objects.json");
    let blockjson: Vec<BlockData> = serde_json::from_str(contents).expect("oops");
    let objectjson: Vec<ObjectData> = serde_json::from_str(objcontents).expect("oops");

    // rename every block n object
    let betterblockjson = blockjson.into_iter().map(|v| {
        let mut y = v;
        y.name = y.name.replace(" ", "_").to_lowercase();
        y
    });
    let betterobjectjson: Vec<ObjectData> = objectjson
        .into_iter()
        .map(|v| {
            let mut y = v;
            y.name = y.name.replace(" ", "_").to_lowercase();
            y
        })
        .collect();

    //filter out rules
    let rulejson: Vec<BlockData> = betterblockjson
        .to_owned()
        .into_iter()
        .filter(|v: &BlockData| v.typ == "rule")
        .collect();
    let actualblockjson: Vec<BlockData> = betterblockjson
        .into_iter()
        .filter(|v| !rulejson.contains(v))
        .collect();

    CompiledData {
        obj: betterobjectjson,
        rules: rulejson,
        blocks: actualblockjson,
    }
}
