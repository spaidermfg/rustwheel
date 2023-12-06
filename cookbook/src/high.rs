use std::rc::Rc;
use std::sync::{Arc, Mutex};
use num::Complex;
use std::time::{Instant,Duration};
use std::string::String;
use std::ops::Add;

pub fn higher() {
    println!();
    handle_csv();
    create();
    real_imaginary();
    let mandelbrot = calculate_mandelbrot(1000, 2.0, 1.0, -1.0, 1.0, 100,24);
    render_mandelbrot(mandelbrot);

    let life = add_with_lifetimes(&3, &5);
    println!("{}", life);

    test_add_portal();
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
    assert_ne!(0.1 + 0.2, 0.3);
}

fn real_imaginary() {
    let a = Complex{ re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.33);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);

    for n in 0..100 {
        if n % 2 == 0 {
            continue;
        }
        println!("n={}", n);
    }

    let mut count = 0;
    let time_limit = Duration::new(1,0);
    let start = Instant::now();
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("count={}", count);

    let mut ins: Vec<i32> = vec![];
    let loop_result = loop {
        ins.push(56);
        if ins.len() > 5000 {
            break 123;
        }
    };
    println!("loop result={}", loop_result);

    let needle = 0o204; // 2 x 8^2 + 0 x 8^1 + 4 x 8^0
    let haystack = [1,2,1,5,23,44,132,456,3221,23445];
    for item in &haystack {
      if *item == needle {
          println!("needle={}", item);
      }
    }
}

fn calculate_mandelbrot(max_iters: usize, x_min:f64, x_max:f64, y_min:f64, y_max:f64,width:usize,height:usize) -> Vec<Vec<usize>>{
    let mut rows = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx,cy,max_iters);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex{ re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);
    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => '#',
                3..=5 => '.',
                6..=10 => '˚',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%'
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

// 显式生命周期注解
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i * *j
}

// 泛型函数
fn add_portal<T: Add<Output = T>>(i: T, j: T) -> T {
    i.add(j)
}

fn test_add_portal() {
    let floats = add_portal(1.3, 3.4);
    let ints = add_portal(29,10);
    let durations = add_portal(Duration::new(5,0), Duration::new(2,0));

    println!("floats={}, ints={}, durations={:?}", floats, ints, durations);
}