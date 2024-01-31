use std::env;
use std::io::Read;
use wasmer::{Module, Store};
use wasmer_wasix::{Pipe, WasiEnv};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(current_dir) = env::current_dir() {
        println!("DIR: {:?}", current_dir);
    } else {
        println!("Failed!")
    }

    let wasm_path = "wasmprobe/examples/hello.wasm";
    let wasm_bytes = std::fs::read(wasm_path)?;
    let mut store = Store::default();

    println!("Compiling module...");
    let module = Module::new(&store, wasm_bytes)?;
    let (stdout_tx, mut stdout_rx) = Pipe::channel();

    WasiEnv::builder("hello")
        .stdout(Box::new(stdout_tx))
        .run_with_store(module, &mut store)?;

    let mut buf = String::new();
    stdout_rx.read_to_string(&mut buf).unwrap();
    eprintln!("Output: {buf}");

    Ok(())
}

#[test]
#[cfg(feature = "wasi")]
fn test_wasi() -> Result<(), Box<dyn std::error::Error>> {
    main()
}
