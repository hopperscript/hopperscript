use rhai::{Engine, EvalAltResult};

pub fn init_block_data(){
    let ngn = Engine::new();
    // file reading needs to be replaced when compiling to wasm
    ngn.run_file("../".into());
}