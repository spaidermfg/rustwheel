//! lifetime,ownership，borrow
#![allow(unused_variables)]
pub fn life_time() {
    println!("{}", "---".repeat(12));
    let sat_a = CubeSat::new(1);
    let sat_b = CubeSat::new(2);
    let sat_c = CubeSat::new(3);

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("{:?} {:?} {:?}", a_status, b_status, c_status);

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

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}
