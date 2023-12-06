use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub fn higher() {
    println!();
    handle_csv();
    create();
}

fn handle_csv() {
    let penguin_data = "\n
    common name,length (cm)\n
    Little penguin,33\n
    Yellow-eyed penguin,65\n
    Fiordland penguin,60\n
    Invalid,data\n
    ";
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        //println!("{} {:?}", i,record);
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

fn create() {
    // 存储在栈中
    let a = 10;

    let e = 25_i32;

    // 存储在堆中
    let b = Box::new(20);

    // 包装在一个引用计数器中的堆中
    let c = Rc::new(Box::new(30));

    // 包装在一个原子引用计数器中，并由一个互斥锁保护
    let d = Arc::new(Mutex::new(40));

    println!("a={:?}
 b={:?}
 c={:?}
 d={:?} e={:?}", a, b, c, d, e);

    let one_million: i64 = 1_000_000;
    println!("one million={}", one_million);

    let binary = 0b11;
    let octal = 0o36;
    let hex = 0x12C;
    println!("base 10: {} {} {}", binary, octal, hex);
    println!("base 2: {:b} {:b} {:b}", binary, octal, hex);
    println!("base 8: {:o} {:o} {:o}", binary,octal,hex);
    println!("base 16: {:X} {:X} {:X}",binary,octal,hex);

    // 避免测试浮点数的相等性
    assert_eq!(0.1 + 0.2, 0.3);
}