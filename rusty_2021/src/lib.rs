// Template
// use crate::lib::Solve;

// pub struct Day7 {}

// impl Day7 {
//     fn solve(input: String) -> String {
//         input
//             .lines()
//             .map(|m| m.parse().unwrap())
//             .collect::<Vec<usize>>()
//             .windows(size)
//             .filter(|m| m.last() > m.first())
//             .count()
//             .to_string()
//     }
// }

// impl Solve for Day7 {
//     fn name() -> String {
//         "The Treachery of Whales".into()
//     }

//     fn part_one(input: String) -> String {
//         Day7::solve(input)
//     }

//     fn part_two(input: String) -> String {
//         Day7::solve(input)
//     }
// }

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

pub trait Solve {
    fn solve(input_path: &str) {
        let input = read_input(input_path);
        println!(
            "{}: {} || {}",
            Self::name(),
            Self::part_one(input.clone()),
            Self::part_two(input)
        )
    }

    fn name() -> String;
    fn part_one(input: String) -> String;
    fn part_two(input: String) -> String;
}

pub fn read_input(file_path: &str) -> String {
    let mut input = String::new();
    let mut file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("Failed to open file: {}", err);
        exit(1);
    });
    file.read_to_string(&mut input).unwrap_or_else(|err| {
        eprintln!("Failed to read file: {}", err);
        exit(1);
    });
    input
}

mod test {

    #[test]
    fn input_day01_2020() {
        let input_test: String = "1721\n979\n366\n299\n675\n1456\n".to_string();
        let input = super::read_input("input/input_day01_2020.txt");

        assert_eq!(input_test, input);
    }
}
