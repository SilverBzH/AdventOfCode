use aoc::Solve;

pub struct Day2 {}

impl Day2 {
    fn parse(input: String) -> Vec<String> {
        input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    fn solve(input: String, advanced: bool) -> String {
        let mut depth = 0;
        let mut horizontal = 0;
        let mut aim = 0;
        for data in Day2::parse(input) {
            let mut split = data.split(' ');
            let (direction, value) = (split.next().unwrap(), split.next().unwrap());
            let value = value.parse::<i32>().unwrap();
            match direction {
                "forward" => {
                    horizontal += value;
                    if advanced {
                        depth += aim * value;
                    }
                }
                "down" => {
                    if advanced {
                        aim += value
                    } else {
                        depth += value
                    }
                }
                "up" => {
                    if advanced {
                        aim -= value
                    } else {
                        depth -= value
                    }
                }
                _ => {}
            }
        }
        let answer = depth * horizontal;
        answer.to_string()
    }
}

impl Solve for Day2 {
    fn name() -> String {
        "Dive !".into()
    }

    fn part_one(input: String) -> String {
        Day2::solve(input, false)
    }

    fn part_two(input: String) -> String {
        Day2::solve(input, true)
    }
}
