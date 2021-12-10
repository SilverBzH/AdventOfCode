mod lib;
use lib::Solve;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

fn main() {
    println!("Advent Of Code 11111100101 !");
    day_01::Day1::solve("input/day_01.txt");
    day_02::Day2::solve("input/day_02.txt");
    day_03::Day3::solve("input/day_03.txt");
    day_04::Day4::solve("input/day_04.txt");
    day_05::Day5::solve("input/day_05.txt");
    day_06::Day6::solve("input/day_06.txt");
    day_07::Day7::solve("input/day_07.txt");
}
