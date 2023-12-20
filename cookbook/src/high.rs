use std::fs::{File, read};
use std::io;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use num::Complex;
use std::time::{Instant,Duration};
use std::ops::Add;
use regex::{Match, Regex};
use clap::{Arg,App};

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
    grep_lite();
    create_array();

    read_file();
    command();

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

fn grep_lite() {
    let ctx_lines = 2;
    let search_term = "animation";
    let quote = "When the play/pause button is clicked,
    check for whether we have the identifier for a queued animation frame.
    If we do,
    then the game is currently playing,
    and we want to cancel the animation frame so that renderLoop isn't called again,
    effectively pausing the game.
    If we do not have an identifier for a queued animation frame,
    then we are currently paused,
    and we would like to call request AnimationFrame to resume the game.";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    let re = Regex::new("animation").unwrap();
    for (i, line) in quote.lines().enumerate() {
        // if line.contains(search_term) {
        //     tags.push(i);
        //
        //     let v = Vec::with_capacity(2 * ctx_lines + 1);
        //     ctx.push(v);
        // }
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => {
                tags.push(i);

                let v = Vec::with_capacity(2 * ctx_lines + 1);
                ctx.push(v);
            },
            None => (),
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i,line) in quote.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }

    println!();
    for (i,line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("{} {}",line_num, line);
        }
    }
}

fn create_array() {
    let one = [1,2,3];
    let two: [u8;3] = [1,2,3];
    let blank1 = [2;3];
    let blank2:[u8;3] = [0; 3];

    let arrays = [one, two, blank1, blank2];
    for a in &arrays {
        print!("{:?}:", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n+10 );
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t(∑{:?} = {})", a, sum);
    }
}

fn command() {
    let args = App::new("grep-lite").version("v1.0.0")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(false)).get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}

fn read_file() {
    let f = File::open("Cargo.toml").unwrap();
    let reader = BufReader::new(f);
    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{}-{} bytes long", line, line.len());
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("find: {}", line),
            None => println!("Is null input"),
        }
    }
}