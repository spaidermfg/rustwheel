use crate::enums::enum_mind;
use crate::param::param_cmd;
use crate::random::rand_num;
use crate::sort::sort_thing;
use crate::tar::tar_handler;

mod random;
mod sort;
mod param;
mod tar;
mod enums;

fn main() {
    println!("Hello, Cookbook!");

    rand_num();

    sort_thing();

    param_cmd();

    tar_handler();

    enum_mind()
}
