use aoc::Solve;

mod y2021;
mod y2023;

fn main() {
    y2023();
}

fn y2023() {
    println!("Advent Of Code 2023 !");
    y2023::day_01::Day1::solve("input/y2023/d1.txt".into());
}

#[allow(dead_code)]
fn y2021() {
    println!("Advent Of Code 11111100101 !");
    y2021::day_01::Day1::solve("input/y2021/day_01.txt");
    y2021::day_02::Day2::solve("input/y2021/day_02.txt");
    y2021::day_03::Day3::solve("input/y2021/day_03.txt");
    y2021::day_04::Day4::solve("input/y2021/day_04.txt");
    y2021::day_05::Day5::solve("input/y2021/day_05.txt");
    y2021::day_06::Day6::solve("input/y2021/day_06.txt");
    y2021::day_07::Day7::solve("input/y2021/day_07.txt");
}
