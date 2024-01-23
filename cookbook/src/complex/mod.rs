#![allow(unused_variables)]

use std::fmt::{Display, Formatter, write};
use ansi_term::unstyled_len;
use rand::prelude::*;
mod impl_three;

pub fn complex_process() {
    println!("[complex type]");

    file_main();

    unsafe_err();

    impl_three::impl_three();

    use_parse_log();

    play_video();
}

// 可变静态全局变量
static mut ERROR: isize =  0;

// 使用普通函数对API进行实验
fn file_main() {
    let vec_data = vec![114, 117,115,116,33];
    let mut f1 = File::new_with_data("hello.txt", &vec_data);

    let mut buffer: Vec<u8> = Vec::new();
    f1 = open(f1).unwrap();
    let f1_length = f1.read(&mut buffer).unwrap();
    f1 = close(f1).unwrap();

    // 将Vec<u8>转换为String，任何无效UTF-8字符都会被转换为特殊字符
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f1);
    println!("name: {} is {} bytes long", &f1.name, f1_length);
    println!("{}", text);
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

// 要为结构体实现trait，结构体中的字段也需要实现该trait
impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File{
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &mut File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }

        let mut tmp = self.data.clone();
        let tmp_length = tmp.len();

        save_to.reserve(tmp_length);
        save_to.append(&mut tmp);  // append会将other vec清空
        Ok(tmp_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    // 执行10000次，有一次失败
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("");
        return Err(err_msg)
    }
    f.state = FileState::Closed;
    Ok(f)
}

// ! 代表函数永不返回

fn unsafe_err() {
    let f = File::new("hello.txt");

    // 访问并修改可变静态变量，必须使用unsafe
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while reading the file.")
        }
    }
    close(f).unwrap();
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while closing the file.")
        }
    }
}

fn one_in(denominator: u32) -> bool {
    // 创建一个线程局部随机数生成器
    // n/m的概率返回布尔值
    thread_rng().gen_ratio(1, denominator)
}

// 枚举体用来表示多个已知的变体
// 枚举支持使用impl块来实现方法
// 枚举体中可以包含数据
// 引入字符串形式数据可以考虑使用枚举类型
#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown
}

type Message = String;

fn parse_log(line: &str) -> (Event, Message) {
    let parts: Vec<_> = line.splitn(2,' ').collect();
    if parts.len() == 1 {
       return (Event::Unknown,String::from(line));
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _  => (Event::Unknown, rest)
    }
}

fn use_parse_log() {
    let log = "BEGIN Hello world!\r
UPDATE update from user set a = 1 where b = 2;\r
delete delete user where a = 4;";
    for line in log.lines() {
        let result = parse_log(line);
        println!("{:?}", result);
    }
}

#[derive(Debug,PartialOrd, PartialEq)]
enum FileState {
    Open,
    Closed,
}

// 为FileState实现Display trait
impl Display for FileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "ClOSE")
        }
    }
}


// trait 让编译器和开发者知道，有多个类型试图执行同一个任务
// trait关键字用于定义一个trait，impl关键字用来给一个具体的类型附加上某个trait
#[derive(Debug)]
struct Video;
trait Play {
    fn begin(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

// 为Video类型实现Play trait
impl Play for Video {
    fn begin(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(67)
    }
}

fn play_video() {
    let v = Video{};
    let mut buffer = vec!();
    let result = v.begin(&mut buffer).unwrap();
    println!("{} bytes read from {:?}", result, v);
}


