use rhai::{Engine, /*EvalAltResult*/};

pub fn init_block_data(){
    let ngn = Engine::new();
    // file reading needs to be replaced when compiling to wasm
    println!("{:?}",ngn.run_file("src/compiler//blockdata.rhai".into()));
}