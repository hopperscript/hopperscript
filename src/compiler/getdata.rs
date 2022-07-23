//use ariadne::{sources, ColorGenerator, Fmt, Label, Report, ReportKind};
use rhai::{Engine, Map, FnPtr, EvalAltResult};

pub fn generate_data_getter() -> () {
    let mut ngn = Engine::new();

    // increase if ExprTooDeep
    ngn.set_max_expr_depths(500, 500);

    let ast = ngn.compile_file("src/compiler/data.rhai".into()).unwrap();

    ngn.register_fn("registerObject", move |name: &str, fun: FnPtr| {
        let e = Engine::new();
        let p: Result<Map, Box<EvalAltResult>> = fun.call(&e, &ast, ());

        println!("{}", name);
        println!("{:#?}", p);
    });

    // file reading needs to be replaced when compiling to wasm
    ngn
        .run_file("src/compiler/data.rhai".into())
        .expect("Error while compiling preset data.");
}
