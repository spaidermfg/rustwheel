use crate::param::param_cmd;
use crate::random::rand_num;
use crate::sort::sort_thing;

mod random;
mod sort;
mod param;

fn main() {
    println!("Hello, Cookbook!");

    rand_num();

    sort_thing();

    param_cmd();
}
