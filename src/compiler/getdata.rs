use rhai::{Engine, Scope, Map, Array};

pub fn generate_data_getter() -> impl Fn(&str, Array)->Map{
    let mut ngn = Engine::new();

    // increase if ExprTooDeep
    ngn.set_max_expr_depths(500, 500);
    
    // file reading needs to be replaced when compiling to wasm
    let ast = ngn.compile_file("src/compiler/blockdata.rhai".into())
    .expect("Error while compiling preset data.");
    let scope = Scope::new();
    move |name: &str, args: Array| -> Map{
        ngn.call_fn(&mut scope.to_owned(), &ast, name, (args,)).unwrap()
    }
}