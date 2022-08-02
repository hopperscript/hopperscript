use rhai::{Array, Engine, EvalAltResult, FnPtr, Scope, AST};
use uuid::Uuid;

pub struct CompiledData {
    pub ast: AST,
    pub obj: Vec<FnPtr>,
    pub eng: Engine,
}

fn uuid() -> Result<String, Box<EvalAltResult>> {
    Ok(Uuid::new_v4().to_string())
}

pub fn generate_data(path: &str) -> CompiledData {
    let mut ngn = Engine::new();

    // increase if ExprTooDeep
    ngn.set_max_expr_depths(500, 500);

    let mut scope = Scope::new();

    scope.push("objects", Array::new());

    ngn.register_result_fn("uuid", uuid);

    let ast = ngn
        .compile_file_with_scope(&mut scope, path.into())
        .expect("Failed to load block data");

    ngn.run_file_with_scope(&mut scope, path.into())
        .expect("Failed to load block data");

    CompiledData {
        obj: scope
            .get("objects")
            .unwrap()
            .to_owned()
            .into_typed_array::<FnPtr>()
            .unwrap(),
        ast,
        eng: ngn,
    }
}
