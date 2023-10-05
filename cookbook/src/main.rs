use crate::random::rand_num;
use crate::sort::sort_thing;

mod random;
mod sort;

fn main() {
    println!("Hello, Cookbook!");

    rand_num();

    sort_thing();
}
