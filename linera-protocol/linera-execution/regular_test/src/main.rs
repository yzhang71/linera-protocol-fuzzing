use std::env;
use std::fs::File;
use std::io::Read;
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

fn process_data(data: &[u8]) {
    let module;
    match Module::new(&create_compilation_engine(), data) {
        Ok(ret) => {module = ret}
        Err(_) => {
            return;
        }
    }
    let compiled_bytecode = module.serialize().unwrap();

    let engine = Engine::headless();
    let store = Store::new(&engine);
    let _ = unsafe { Module::deserialize(&store, compiled_bytecode)}.unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let mut file = File::open(input_file).expect("Failed to open input file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read input file");
    process_data(&data);
}
