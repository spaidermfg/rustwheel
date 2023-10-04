use rand::distributions::{Alphanumeric, Distribution};
use rand::distributions::{Standard, Uniform};
use rand::{Rng, thread_rng};
use rand_distr::{Normal, NormalError};

pub fn rand_num() {
    println!("------------------------------------rand num");
    rand1();

    println!("------------------------rand2");
    rand2();

    println!("------------------------rand3");
    rand3().expect("rand error");

    println!("------------------------rand4");
    rand4();

    println!("------------------------rand5");
    rand5();
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

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: bool,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rx, ry, rz) = rng.gen();
        Point {
            x: rx,
            y: ry,
            z: rz,
        }
    }
}

// 生成自定义类型随机值
fn rand4() {
    let mut rng = thread_rng();
    let rand_tuple = rng.gen::<(i32, f64, bool)>();
    let rand_point: Point = rng.gen();

    println!("Random point: {:?}", rand_tuple);
    println!("Random point: {:?}", rand_point);
}

// 生成随机密码
fn rand5() {
    let rng = thread_rng();
    let iter = rng.sample_iter(&Alphanumeric);
    let x: String = iter.take(30).map(char::from).collect();

    println!("{}", x);
}