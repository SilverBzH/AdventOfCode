use aoc::Solve;

pub struct Day3 {}

impl Day3 {
    fn counter(input: String) -> (Vec<u8>, usize) {
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
        let counter = counter.iter().map(|c| if *c > 0 { 1 } else { 0 }).collect();
        (counter, len)
    }

    fn solve(input: String) -> String {
        let (counter, _) = Day3::counter(input.clone());
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

    fn count(data: Vec<&str>, i: usize, o2: bool) -> u32 {
        let ones = data
            .clone()
            .into_iter()
            .filter(|data| data.chars().nth(i).unwrap() == '1')
            .count();
        let zeros = data.len() - ones;
        if ones > zeros {
            if o2 {
                1
            } else {
                0
            }
        } else if zeros > ones {
            if o2 {
                0
            } else {
                1
            }
        } else {
            if o2 {
                1
            } else {
                0
            }
        }
    }

    fn get_info(mut data: Vec<&str>, len: usize, o2: bool) -> u32 {
        let mut comparator: u32 = Day3::count(data.clone(), 0, o2);
        data = data
            .into_iter()
            .filter(|line| line.chars().nth(0).unwrap().to_digit(2).unwrap() == comparator)
            .collect();
        for i in 1..len {
            comparator = Day3::count(data.clone(), i as usize, o2);
            data = data
                .into_iter()
                .filter(|line| {
                    line.chars().nth(i as usize).unwrap().to_digit(2).unwrap() == comparator
                })
                .collect();
            if data.len() == 1 {
                break;
            }
        }
        u32::from_str_radix(data[0], 2).unwrap()
    }

    fn solve2(input: String) -> String {
        let len: usize = input.lines().peekable().peek().unwrap().len();

        let oxygen: Vec<&str> = input.lines().collect();
        let co2: Vec<&str> = input.lines().collect();

        let oxygen = Day3::get_info(oxygen, len, true);
        let co2 = Day3::get_info(co2, len, false);
        let life_support_rating = oxygen * co2;

        life_support_rating.to_string()
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
        Day3::solve2(input)
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

    #[test]
    fn day3_2() {
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
        assert_eq!("230", Day3::part_two(input));
    }
}
