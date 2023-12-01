use aoc::Solve;
use std::collections::HashMap;

pub struct Day5 {}

#[derive(Debug)]
struct Line {
    points: Vec<Point>,
}

impl Line {
    fn from_points(p1: Point, p2: Point, part_two: bool) -> Line {
        let mut points: Vec<Point> = Vec::new();
        if p1.x == p2.x {
            // Vertical
            let (ymax, ymin) = if p1.y > p2.y {
                (p1.y, p2.y)
            } else {
                (p2.y, p1.y)
            };
            let x = p1.x;
            for y in ymin..=ymax {
                points.push(Point::make(x, y));
            }
        } else if p1.y == p2.y {
            // Horizontal
            let (xmax, xmin) = if p1.x > p2.x {
                (p1.x, p2.x)
            } else {
                (p2.x, p1.x)
            };
            let y = p1.y;
            for x in xmin..=xmax {
                points.push(Point::make(x, y));
            }
        } else if part_two {
            let xcap = p2.x;
            let ycap = p2.y;
            let mut xmaker = p1.x;
            let mut ymaker = p1.y;
            points.push(p1.clone());
            loop {
                if xmaker == xcap && ymaker == ycap {
                    break;
                }
                if xmaker > xcap {
                    xmaker -= 1
                } else {
                    xmaker += 1
                };
                if ymaker > ycap {
                    ymaker -= 1
                } else {
                    ymaker += 1
                }
                let point = Point::make(xmaker, ymaker);
                points.push(point);
            }
        }

        Line { points }
    }
}

#[derive(Debug, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

impl Point {
    fn make(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

fn parse(input: String, part_two: bool) -> Vec<Line> {
    input
        .replace(" -> ", " ")
        .replace(",", " ")
        .replace("\n", " ")
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<isize>>()
        .windows(4)
        .step_by(4)
        .map(|coordinates| {
            let p1 = Point::make(coordinates[0], coordinates[1]);
            let p2 = Point::make(coordinates[2], coordinates[3]);
            Line::from_points(p1, p2, part_two)
        })
        .collect::<Vec<Line>>()
}

impl Day5 {
    fn solve(input: String, part_two: bool) -> String {
        let lines = parse(input, part_two);
        let mut match_points: HashMap<Point, usize> = HashMap::new();

        for line in lines {
            for point in line.points {
                if let Some(crossed) = match_points.get_mut(&point) {
                    *crossed += 1;
                } else {
                    match_points.insert(point, 1);
                }
            }
        }

        let crossed: HashMap<&Point, &usize> = match_points
            .iter()
            .filter(|(_, crossed)| **crossed > 1)
            .collect();
        crossed.len().to_string()
    }
}

impl Solve for Day5 {
    fn name() -> String {
        "Hydrothermal Venture".into()
    }

    fn part_one(input: String) -> String {
        Day5::solve(input, false)
    }

    fn part_two(input: String) -> String {
        Day5::solve(input, true)
    }
}
