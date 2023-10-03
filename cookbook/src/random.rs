use rand::distributions::Uniform;
use rand::distributions::Distribution;
use rand::{Rng};
use rand_distr::{Normal, NormalError};


pub fn rand_num() {
    println!("------------------------------------rand num");
    rand1();

    println!("------------------------rand2");
    rand2();

    println!("------------------------rand3");
    rand3().expect("TODO: panic message");
}

// 生成随机数
fn rand1() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

// 生成范围内随机数
fn rand2() {
    let mut rng = rand::thread_rng();

    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));

    let mut rng1 = rand::thread_rng();
    let die = Uniform::from(1..40);
    loop {
        let throw = die.sample(&mut rng1);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

// 生成给定分布随机数
fn rand3() -> Result<(), NormalError> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);

    println!("{} is from a N(2, 9) distribution", v);
    Ok(())
}
