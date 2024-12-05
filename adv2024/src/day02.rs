use anyhow;
use std::fs::File;
use std::io::{self, BufRead};

fn parse() -> anyhow::Result<Vec<Vec<usize>>> {
    let input = File::open("input/day02.txt")?;
    let reader = io::BufReader::new(input);

    let reports: Vec<Vec<usize>> = reader
        .lines()
        .map(|report| {
            report
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    Ok(reports)
}

fn safe_reports(reports: Vec<Vec<usize>>, checker: impl Fn(Vec<usize>) -> bool) -> Vec<Vec<usize>> {
    let safe_reports = reports
        .into_iter()
        .filter(|report| checker(report.clone()))
        .collect::<Vec<Vec<usize>>>();

    safe_reports
}

fn is_in_bound(report: &Vec<usize>) -> bool {
    for i in 1..report.len() {
        let diff = report[i - 1].abs_diff(report[i]);
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn is_increasing(report: &Vec<usize>) -> bool {
    report.is_sorted()
}

fn is_decreasing(report: &Vec<usize>) -> bool {
    let mut report = report.clone();
    report.reverse();
    report.is_sorted()
}

fn part01() -> anyhow::Result<usize> {
    let reports = parse()?;
    let checker = |report: Vec<usize>| -> bool {
        is_in_bound(&report) && (is_increasing(&report) || is_decreasing(&report))
    };

    Ok(safe_reports(reports, checker).len())
}

fn part02() -> anyhow::Result<usize> {
    let reports = parse()?;
    let checker = |report: Vec<usize>| -> bool {
        let mut is_safe =
            is_in_bound(&report) && (is_increasing(&report) || is_decreasing(&report));
        if !is_safe {
            for i in 0..report.len() {
                let mut ret = report.clone();
                ret.remove(i);
                is_safe = is_in_bound(&ret)
                    && (is_increasing(&ret) || is_decreasing(&ret));
                if is_safe {
                    break;
                }
            }
        }
        is_safe
    };

    Ok(safe_reports(reports, checker).len())
}

pub fn print_result() {
    println!("{}", part01().unwrap());
    println!("{}", part02().unwrap());
}
