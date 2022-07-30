//use ariadne::{sources, ColorGenerator, Fmt, Label, Report, ReportKind};
use rhai::{Engine, FnPtr, Scope, Array, AST};

pub struct CompiledData {
    pub ast: AST,
    pub obj: Vec<FnPtr>,
    pub eng: Engine
}

pub fn generate_data() -> CompiledData {
    let mut ngn = Engine::new();

    // increase if ExprTooDeep
    ngn.set_max_expr_depths(500, 500);

    let mut scope = Scope::new();

    scope.push("objects", Array::new());

    let ast = ngn.compile_file_with_scope(&mut scope, "src/compiler/data.rhai".into()).expect("Failed to load block data");

    ngn.run_file_with_scope(&mut scope, "src/compiler/data.rhai".into()).expect("Failed to load block data");

    CompiledData {
        obj: scope.get("objects").unwrap().to_owned().into_typed_array::<FnPtr>().unwrap(),
        ast,
        eng: ngn
    }
}
