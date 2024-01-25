//! lifetime,ownership，borrow
#![allow(unused_variables)]
pub fn life_time() {
    println!("{}", "---".repeat(12));
    let sat_a = 1;
    let sat_b = 2;
    let sat_c = 3;

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("{:?} {:?} {:?}", a_status, b_status, c_status);
}



#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: i64) -> StatusMessage {
    StatusMessage::Ok
}
