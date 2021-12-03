mod lib;
use lib::Solve;

mod day_01;
mod day_02;
mod day_03;

fn main() {
    println!("Advent Of Code 11111100101 !");
    day_01::Day1::solve("input/day_01.txt");
    day_02::Day2::solve("input/day_02.txt");
    day_03::Day3::solve("input/day_03.txt");
}
