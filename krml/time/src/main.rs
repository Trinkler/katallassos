mod lib;
use lib::*;

fn main() {
    let x = Time::new(2020, 6, 14, 4, 25, 56);
    let y = Time::new(2019, 6, 15, 4, 25, 56);
    println!("{:?}", x < y);
}
