use wasmer::{imports, Instance, Module, Store, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let module_wat = r#"
    (module
      (type $t0 (func (param i32) (result i32)))
      (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
        get_local $p0
        i32.const 8
        i32.add))
    "#;

    let mut store = Store::default();
    let import_object = imports! {};
    let module = Module::new(&store, &module_wat)?;
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;
    let res = add_one.call(&mut store, &[Value::I32(42)])?;

    println!("result: {:?}", res[0]);

    Ok(())
}

#[test]
fn test_add() -> Result<(), Box<dyn std::error::Error>> {
    main()
}