use rhai::{Engine, /*EvalAltResult*/};

pub fn init_block_data(){
    let mut ngn = Engine::new();

    // increase if ExprTooDeep
    ngn.set_max_expr_depths(500, 500);
    
    // file reading needs to be replaced when compiling to wasm
    println!("{:?}",ngn.run_file("src/compiler//blockdata.rhai".into()));
}