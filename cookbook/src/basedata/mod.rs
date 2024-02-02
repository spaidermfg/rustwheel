///! 基础数据类型
/// # 不同语法调用不同的trait
/// * {} -> std::fmt::Display;
/// * {:?} -> std::fmt::Debug;
/// * {032b} -> std::fmt::Binary;
/// * 使用unsafe块就是表示自己对这段代码负责，编译器不提供内存安全的保证(慎用)


pub fn base_date() {
    println!("{}base_data", "-----".repeat(10));
    bit();
}

fn bit() {
    let a: u16 = 50115;
    let b: i16 = -15421;
    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);

    let c: f32 = 67.67;
    // unsafe块中非安全操作
    // 将f32转换为u32类型
    let frankfurter: u32 = unsafe {
        std::mem::transmute(c)
    };

    println!("c: {} {}", c, frankfurter);

    // unsafe块中非安全操作
    // 将u32转换为f32类型
    let d: f32 = unsafe{
        std::mem::transmute(frankfurter)
    };
    println!("d: {}", d);
    assert_eq!(c, d)
}

