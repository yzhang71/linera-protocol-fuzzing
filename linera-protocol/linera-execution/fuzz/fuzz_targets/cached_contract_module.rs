#![no_main]

use libfuzzer_sys::fuzz_target;
use wasmer::{
    CompilerConfig, Engine, EngineBuilder, Module,
    Singlepass, Store,
};
use std::sync::Arc;
use wasmer_middlewares::metering::Metering;
use linera_execution::WasmApplication;

fn create_compilation_engine() -> Engine {
    let metering = Arc::new(Metering::new(0, WasmApplication::operation_cost));
    let mut compiler_config = Singlepass::default();
    compiler_config.push_middleware(metering);
    compiler_config.canonicalize_nans(true);

    EngineBuilder::new(compiler_config).into()
}

fuzz_target!(|data: &[u8]| {

    let module;
    match Module::new(&create_compilation_engine(), data) {
        Ok(ret) => {module = ret}
        Err(_) => {
            return;
        }
    }
    // let module = Module::new(&create_compilation_engine(), data).unwrap();
    let compiled_bytecode = module.serialize().unwrap();

    let engine = Engine::headless();
    let store = Store::new(&engine);
    let module = unsafe { Module::deserialize(&store, compiled_bytecode)}.unwrap();
});

