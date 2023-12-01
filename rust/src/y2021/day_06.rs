use aoc::Solve;

pub struct Day6 {}

#[derive(Debug, Copy, Clone)]
struct Fish {
    brothers: usize,
    timer: usize,
}

impl Fish {
    fn new(timer: &str) -> Self {
        Fish {
            brothers: 0,
            timer: timer.parse().unwrap(),
        }
    }

    fn make(brothers: usize) -> Self {
        let brothers = if brothers == 1 { 0 } else { brothers };
        Fish { timer: 8, brothers }
    }

    fn produce(&mut self) -> (bool, usize) {
        if self.timer == 0 {
            self.timer = 6;
            let brothers = if self.brothers == 0 { 1 } else { self.brothers };
            (true, brothers)
        } else {
            self.timer -= 1;
            (false, 0)
        }
    }

    fn count(&self) -> usize {
        if self.brothers == 0 {
            1
        } else {
            self.brothers
        }
    }
}

impl Day6 {
    fn solve(input: String, part_two: bool) -> String {
        let mut fishes = input
            .split(",")
            .map(|x| Fish::new(x))
            .collect::<Vec<Fish>>();

        let days = if part_two { 256 } else { 80 };
        let mut babies = 0;

        for _ in 0..days {
            for fish in &mut fishes {
                let (is_produce, nb_babies) = fish.produce();
                if is_produce {
                    babies += nb_babies;
                }
            }
            if babies != 0 {
                fishes.push(Fish::make(babies));
            }
            babies = 0;
        }

        fishes
            .iter()
            .map(|fish| fish.count())
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>()
            .to_string()
    }
}

impl Solve for Day6 {
    fn name() -> String {
        "Lanternfish".into()
    }

    fn part_one(input: String) -> String {
        Day6::solve(input, false)
    }

    fn part_two(input: String) -> String {
        Day6::solve(input, true)
    }
}
