#![allow(unused_variables)]
pub fn complex_process() {
    println!("complex type");

    file_main();
}


// 使用普通函数对API进行实验
fn file_main() {
    let mut f1 = File::from("a.txt");
    open(&mut f1);
    close(&mut f1);
}

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

// ! 代表函数永不返回
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

