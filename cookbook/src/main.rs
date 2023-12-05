use crate::enums::enum_mind;
use crate::high::higher;
use crate::param::param_cmd;
use crate::random::rand_num;
use crate::sort::sort_thing;
use crate::tar::tar_handler;

mod random;
mod sort;
mod param;
mod tar;
mod enums;
mod high;

fn main() {
    println!("Hello, Cookbook!");

    rand_num();

    sort_thing();

    param_cmd();

    tar_handler();

    enum_mind();

    higher();
}
