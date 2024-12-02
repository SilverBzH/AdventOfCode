use std::fs::File;
use std::io::{self, BufRead};

fn parse() -> (Vec<usize>, Vec<usize>) {
    let input = File::open("input/day01_1.txt").unwrap();
    let reader = io::BufReader::new(input);

    let mut left = Vec::<usize>::new();
    let mut right = Vec::<usize>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        left.push(parts[0].trim().parse::<usize>().unwrap());
        right.push(parts[1].trim().parse::<usize>().unwrap());
    }

    (left, right)
}

fn part01() -> usize {
    let mut ans = Vec::<usize>::new();
    let (mut left, mut right) = parse();

    left.sort();
    left.reverse();

    right.sort();
    right.reverse();

    loop {
        let l = left.pop().unwrap();
        let r = right.pop().unwrap();
        ans.push(l.abs_diff(r));
        if right.is_empty() {
            break;
        }
    }

    ans.iter().sum::<usize>()
}

fn part02() -> usize {
    let mut ans = Vec::<usize>::new();
    let (mut left, right) = parse();

    loop {
        let l = left.pop().unwrap();
        let occurence = right.iter().filter(|x| **x == l).count();
        ans.push(l * occurence);

        if left.is_empty() {
            break;
        }
    }

    ans.iter().sum::<usize>()
}

pub fn print_result() {
    println!("{}", part01());
    println!("{}", part02());
}
