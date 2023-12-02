use std::os::unix::process;

use aoc::Solve;

pub struct Day1 {}

impl Day1 {
    fn solve1(input: String) -> String {
        let calibration_values: Vec<usize> = input
            .lines()
            .map(|line| {
                // should be the calibration value
                let mut digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
                let first = digits[0];
                let last = digits.pop().unwrap();
                let calibration_value: usize = format!("{first}{last}").parse().unwrap();
                calibration_value
            })
            .collect();
        calibration_values.iter().sum::<usize>().to_string()
    }

    fn solve2(input: String) -> String {
        let possible_values = vec![
            "1", "one", "2", "two", "three", "3", "four", "4", "five", "5", "six", "6", "seven",
            "7", "eight", "8", "nine", "9",
        ];

        let calibration_values: Vec<usize> = input
            .lines()
            .map(|line| {
                let fo = first_occurence(line.into(), &possible_values);
                let lo = last_occurence(line.into(), &possible_values);
                format!("{fo}{lo}").parse().unwrap()
            })
            .collect();

        calibration_values.iter().sum::<usize>().to_string()
    }
}

fn first_occurence(input: String, possible_values: &Vec<&str>) -> String {
    let mut i = 0;
    loop {
        if i == input.len() {
            break;
        }
        for value in possible_values {
            if input[i..].starts_with(value) {
                return convert(*value).into();
            }
        }
        i += 1;
    }
    "0".into()
}

fn last_occurence(input: String, possible_values: &Vec<&str>) -> String {
    let mut i: isize = (input.len() - 1) as isize;
    loop {
        if i == -1 {
            break;
        }
        for value in possible_values {
            let index = i as usize;
            if input[..=index].ends_with(value) {
                return convert(*value).into();
            }
        }
        i -= 1;
    }
    "0".into()
}

fn convert(value: &str) -> &str {
    match value {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => value,
    }
}

impl Solve for Day1 {
    fn name() -> String {
        "Trebuchet?!".into()
    }

    fn part_one(input: String) -> String {
        // Day1::solve1(input)
        String::new()
    }

    fn part_two(input: String) -> String {
        Day1::solve2(input)
    }
}
