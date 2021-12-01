mod lib;
use lib::read_input;
mod day_01;

fn main() {
    println!("Advent Of Code 11111100101 !");
    day_01::solve(read_input("input/day_01.txt"));
}
