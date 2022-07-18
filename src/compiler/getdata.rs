use ariadne::{sources, ColorGenerator, Fmt, Label, Report, ReportKind};
use rhai::{Array, Engine, Map, Scope};

pub fn generate_data_getter() -> impl Fn(&str, Array, usize, &str) -> Map {
    let mut ngn = Engine::new();

    // increase if ExprTooDeep
    ngn.set_max_expr_depths(500, 500);

    // file reading needs to be replaced when compiling to wasm
    let ast = ngn
        .compile_file("src/compiler/data.rhai".into())
        .expect("Error while compiling preset data.");
    let scope = Scope::new();
    move |name: &str, args: Array, ln: usize, line: &str| -> Map {
        let val = ngn.call_fn(&mut scope.to_owned(), &ast, name, (args,));
        if val.is_ok() {
            //ariadne error
            Report::build(ReportKind::Error, "TODO: filename", ln);
        }
        val.unwrap()
    }
}
