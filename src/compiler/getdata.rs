use rhai::{Engine, AST, EvalAltResult};

pub fn init_block_data(){
    let mut ngn = Engine::new();

    // increase if ExprTooDeep
    ngn.set_max_expr_depths(500, 500);
    
    // file reading needs to be replaced when compiling to wasm
    let ast = ngn.compile_file("src/compiler/blockdata.rhai".into())
    .expect("Error while compiling preset data.");
    
}