///! 基础数据类型
///! # 不同语法调用不同的trait
///! * {} -> std::fmt::Display;
///! * {:?} -> std::fmt::Debug;
///! * {032b} -> std::fmt::Binary;
///! * 使用unsafe块就是表示自己对这段代码负责，编译器不提供内存安全的保证(慎用)
///! # 整数溢出
///! * 递增一个整数并超出其允许的范围
///! # 字节序 endianness
///！* 整数字节排列的顺序
///


pub fn base_date() {
    println!("{}base_data", "-----".repeat(10));
    bit();
}

fn bit() {
    // 基础类型位模式一致，类型不同表示的值不同
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
    assert_eq!(c, d);

    // 模拟整数溢出
    // let mut i: u16 = 0;
    // print!("{}->", i);
    // loop {
    //     i += 1000;
    //     print!("{}->", i);
    //     if i % 10000 == 0 {
    //         print!("\n");
    //     }
    // }

    // 将u16位模式转换为整数值
    let zero: u16 = 0b0000_0000_0000_0000;
    let one: u16 = 0b0000_0000_0000_0001;
    let two: u16 = 0b0000_0000_0000_0010;

    let sixtyfivethousand_533: u16 = 0b1111_1111_1111_1101;
    let sixtyfivethousand_534: u16 = 0b1111_1111_1111_1110;
    let sixtyfivethousand_535: u16 = 0b1111_1111_1111_1111;

    print!("{} {} {}....", zero, one, two);
    println!("{} {} {}", sixtyfivethousand_533, sixtyfivethousand_534, sixtyfivethousand_535);
}

