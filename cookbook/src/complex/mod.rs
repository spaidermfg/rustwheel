#![allow(unused_variables)]

pub fn complex_process() {
    println!("[complex type]");

    file_main();
}

// 可变静态全局变量
static mut ERROR: i32 =  0;

// 使用普通函数对API进行实验
fn file_main() {
    let vec_data = vec![114, 117,115,116,33];
    let mut f1 = File::new_with_data("hello.txt", &vec_data);

    let mut buffer: Vec<u8> = Vec::new();
    open(&mut f1);
    let f1_length = f1.read(&mut buffer);
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

impl File {
    fn new(name: &str) -> File {
        File{
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &mut File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let tmp_length = tmp.len();

        save_to.reserve(tmp_length);
        save_to.append(&mut tmp);  // append会将other vec清空
        tmp_length
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

// ! 代表函数永不返回

fn unsafe_err() {
    let mut f = File::new("hello.txt");

    // 访问并修改可变静态变量，必须使用unsafe
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while reading the file.")
        }
    }
    close(&mut f);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while closing the file.")
        }
    }
}