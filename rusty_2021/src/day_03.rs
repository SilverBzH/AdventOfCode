use crate::lib::Solve;

pub struct Day3 {}

impl Day3 {
    fn solve(input: String) -> String {
        let len: usize = input.lines().peekable().peek().unwrap().len();
        let mut counter: Vec<i8> = vec![0; len];
        for diag in input.lines() {
            let digit: Vec<char> = diag.chars().collect();
            for i in 0..len {
                match digit[i] {
                    '0' => counter[i] -= 1,
                    '1' => counter[i] += 1,
                    _ => unimplemented!(),
                }
            }
        }
        let gamma: Vec<u8> = counter.iter().fold(Vec::new(), |mut gamma, c| {
            if *c > 0 {
                gamma.push(1);
            } else {
                gamma.push(0);
            }
            gamma
        });
        let epsilon: Vec<u8> = gamma
            .iter()
            .map(|bit| if *bit > 0 { 0 } else { 1 })
            .collect();
        let diag = Day3::to_int(&gamma) * Day3::to_int(&epsilon);
        diag.to_string()
    }

    fn to_int(data: &[u8]) -> i32 {
        let mut decimal = 0;
        let base: i32 = 2;
        let mut i = 0;
        data.iter().rev().for_each(|bit| {
            if *bit > 0 {
                decimal += base.pow(i);
            };
            i += 1;
        });
        decimal
    }
}

impl Solve for Day3 {
    fn name() -> String {
        "Binary Diagnostic".into()
    }

    fn part_one(input: String) -> String {
        Day3::solve(input)
    }

    fn part_two(input: String) -> String {
        Day3::solve(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day3_1() {
        let input: String = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            .into();
        assert_eq!("198", Day3::part_one(input));
    }
}
