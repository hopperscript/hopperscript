use rhai::{Array, Engine, EvalAltResult, FnPtr, Scope, AST, Map, serde::{to_dynamic, from_dynamic}, Dynamic};
use uuid::Uuid;

use crate::compiler::Value;

pub struct CompiledData {
    pub ast: AST,
    pub obj: Vec<FnPtr>,
    pub eng: Engine,
    pub rules: Vec<FnPtr>,
    pub blocks: Vec<FnPtr>,
}

fn uuid() -> Result<String, Box<EvalAltResult>> {
    Ok(Uuid::new_v4().to_string())
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
    
    if val.value.is_some() {
        map.insert("value".into(), to_dynamic(val.value).unwrap());
    }

    Ok(map)
}

pub fn generate_data(path: &str) -> CompiledData {
    let mut ngn = Engine::new();

    // increase if ExprTooDeep
    ngn.set_max_expr_depths(500, 500);

    let mut scope = Scope::new();

    scope.push("objects", Array::new());
    scope.push("rules", Array::new());
    scope.push("blocks", Array::new());

    ngn.register_result_fn("paramset", paramset);

    ngn.register_result_fn("uuid", uuid);

    let ast = ngn
        .compile_file_with_scope(&mut scope, path.into())
        .expect("Failed to load block data");

    ngn.run_file_with_scope(&mut scope, path.into())
        .expect("Failed to load block data");

    CompiledData {
        obj: get_fnptr_list("objects", &scope),
        rules: get_fnptr_list("rules", &scope),
        blocks: get_fnptr_list("blocks", &scope),
        ast,
        eng: ngn,
    }
}
