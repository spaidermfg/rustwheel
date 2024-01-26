//! lifetime,ownership，borrow
//! Rust中基本类型实现了Copy trait，所以可以被隐式借用
//! 基本类型都具有复制语义，其他类型都具有移动语义
#![allow(unused_variables)]
pub fn life_time() {
    println!("{}", "---".repeat(12));
    let sat_a = CubeSat::new(1);
    let sat_b = CubeSat::new(2);
    let sat_c = CubeSat::new(3);

    /// 当值还没有被借用时，重新绑定该值是合法的
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
    println!("{:?} {:?} {:?}", sat_a, sat_b, sat_c);

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("{:?} {:?} {:?}", a_status, b_status, c_status);
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn new(sat_id: u64) -> CubeSat {
        CubeSat{ id: sat_id}
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?} {:?}", sat_id, StatusMessage::Ok);
    sat_id
}
