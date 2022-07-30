use ariadne::{Label, Report, ReportKind, Source, Color, Fmt};
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
        let val = ngn.call_fn(&mut scope.to_owned(), &ast, name, ("Moveforward",""));
        if val.is_err() {
            //ariadne error
            
            println!("{:#?}", val);
            Report::build(ReportKind::Error, (), 0)
            .with_message("No such block")
            .with_label(Label::new(ln..ln+name.len())
            .with_message(format!("Block \"{}\" not found", name).fg(Color::Red))
        .with_color(Color::Red))
            .finish()
            .print(Source::from(format!("{}{}","\n".repeat(ln),line))).unwrap();panic!();
        };
        val.unwrap()
    }
}
