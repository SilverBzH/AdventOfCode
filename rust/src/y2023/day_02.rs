use aoc::Solve;

pub struct Day2 {}

impl Day2 {
    // 2386 -> too low
    fn solve1(input: String) -> String {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        let games: Vec<String> = input
            .lines()
            .map(|line| {
                let index = line.find(':').unwrap();
                line[index + 1..].into()
            })
            .collect();

        let mut i = 0;
        let mut allowed_games = Vec::new();
        for game in games {
            i += 1;

            let mut is_valid_game = true;
            let sets: Vec<&str> = game.split(';').collect();
            for set in sets.iter() {
                let cubes: Vec<&str> = set.split(',').collect();
                let (red, green, blue) = count_cubes(cubes);
                if red > max_red || green > max_green || blue > max_blue {
                    is_valid_game = false;
                }
            }
            if is_valid_game {
                allowed_games.push(i);
            }
        }
        allowed_games.iter().sum::<usize>().to_string()
    }

    fn solve2(input: String) -> String {
        let games: Vec<String> = input
            .lines()
            .map(|line| {
                let index = line.find(':').unwrap();
                line[index + 1..].into()
            })
            .collect();

        let mut power_games = Vec::new();
        for game in games {
            let (mut mred, mut mgreen, mut mblue) = (0, 0, 0);
            let sets: Vec<&str> = game.split(';').collect();
            for set in sets.iter() {
                let cubes: Vec<&str> = set.split(',').collect();
                let (red, green, blue) = count_cubes(cubes);
                if red > mred {
                    mred = red;
                }
                if green > mgreen {
                    mgreen = green;
                }
                if blue > mblue {
                    mblue = blue;
                }
            }
            power_games.push(mred * mgreen * mblue);
        }

        power_games.iter().sum::<usize>().to_string()
    }
}

fn count_cubes(cubes: Vec<&str>) -> (usize, usize, usize) {
    //rgb
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cube in cubes {
        let value: usize = cube
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        if cube.contains("blue") {
            blue += value
        } else if cube.contains("red") {
            red += value
        } else if cube.contains("green") {
            green += value
        }
    }

    (red, green, blue)
}

impl Solve for Day2 {
    fn name() -> String {
        "Cube Conundrum".into()
    }

    fn part_one(input: String) -> String {
        Day2::solve1(input)
    }

    fn part_two(input: String) -> String {
        Day2::solve2(input)
    }
}
