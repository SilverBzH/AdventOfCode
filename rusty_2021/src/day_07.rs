use crate::lib::Solve;

pub struct Day7 {}

impl Day7 {
    fn solve(input: String, part_two: bool) -> String {
        let crabs: Vec<isize> = input.split(',').map(|x| x.parse().unwrap()).collect();
        let mut pivot;
        let mut fuel = 0;
        let mut fuels: Vec<isize> = Vec::new();
        for i in 0..*crabs.iter().max().unwrap() {
            pivot = i;
            for crab in &crabs {
                let moves = (pivot - crab).abs();
                if part_two {
                    fuel += compute_fuel(moves);
                } else {
                    fuel += moves;
                }
            }
            fuels.push(fuel);
            fuel = 0;
        }

        fuels.iter().min().unwrap().to_string()
    }
}

fn compute_fuel(num: isize) -> isize {
    match num {
        0 => 0,
        1 => 1,
        _ => compute_fuel(num - 1) + num,
    }
}

impl Solve for Day7 {
    fn name() -> String {
        "The Treachery of Whales".into()
    }

    fn part_one(input: String) -> String {
        Day7::solve(input, false)
    }

    fn part_two(input: String) -> String {
        Day7::solve(input, true)
    }
}
