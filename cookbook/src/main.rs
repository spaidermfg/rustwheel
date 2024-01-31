use crate::enums::enum_mind;
use crate::high::higher;
use crate::param::param_cmd;
use crate::random::rand_num;
use crate::sort::sort_thing;
use crate::tar::tar_handler;
use std::fmt::Debug;

mod complex;
mod enums;
mod high;
mod lifetime;
mod param;
mod random;
mod sort;
mod tar;

fn main() {
    println!("Hello, Cookbook!");

    rand_num();

    sort_thing();

    param_cmd();

    tar_handler();

    enum_mind();

    higher();

    complex::complex_process();

    lifetime::life_time();
}
