use rhai::{Array, Engine, EvalAltResult, FnPtr, Scope, AST};
use uuid::Uuid;

pub struct CompiledData {
    pub ast: AST,
    pub obj: Vec<FnPtr>,
    pub eng: Engine,
    pub rules: Vec<FnPtr>,
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

pub fn generate_data(path: &str) -> CompiledData {
    let mut ngn = Engine::new();

    // increase if ExprTooDeep
    ngn.set_max_expr_depths(500, 500);

    let mut scope = Scope::new();

    scope.push("objects", Array::new());
    scope.push("rules", Array::new());

    ngn.register_result_fn("uuid", uuid);

    let ast = ngn
        .compile_file_with_scope(&mut scope, path.into())
        .expect("Failed to load block data");

    ngn.run_file_with_scope(&mut scope, path.into())
        .expect("Failed to load block data");

    CompiledData {
        obj: get_fnptr_list("objects", &scope),
        rules: get_fnptr_list("rules", &scope),
        ast,
        eng: ngn,
    }
}
