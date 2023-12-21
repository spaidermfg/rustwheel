#![allow(unused_variables)]

use std::fs::read_to_string;

pub fn complex_process() {
    println!("[complex type]");

    file_main();
}


// 使用普通函数对API进行实验
fn file_main() {
    let mut f1 = File{
        name: String::from("hello.txt"),
        data: vec![114, 117,115,116,33],
    };

    let mut buffer: Vec<u8> = Vec::new();
    open(&mut f1);
    let f1_length = read(&mut f1,&mut buffer);
    close(&mut f1);

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
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

// ! 代表函数永不返回
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let tmp_length = tmp.len();

    save_to.reserve(tmp_length);
    // append会将other vec清空
    save_to.append(&mut tmp);
    tmp_length
}

