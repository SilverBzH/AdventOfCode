use aoc::Solve;

pub struct Day1 {}

impl Day1 {
    fn solve(input: String, size: usize) -> String {
        input
            .lines()
            .map(|m| m.parse().unwrap())
            .collect::<Vec<usize>>()
            .windows(size)
            .filter(|m| m.last() > m.first())
            .count()
            .to_string()
    }
}

impl Solve for Day1 {
    fn name() -> String {
        "Sonar Sweep".into()
    }

    fn part_one(input: String) -> String {
        Day1::solve(input, 2)
    }

    fn part_two(input: String) -> String {
        Day1::solve(input, 4)
    }
}
