pub fn solve(input: String) {
    let m = input.lines().map(|m| m.parse::<i32>().unwrap());
    solve1(m.clone());
    solve2(m);
}

fn solve1(mut measurements: impl Iterator<Item = i32>) {
    let mut prev: i32 = measurements.next().unwrap();
    println!(
        "day_01: {}",
        measurements
            .filter(|m| {
                let ret = m > &prev;
                prev = *m;
                return ret;
            })
            .count()
    );
}

fn solve2(input: impl Iterator<Item = i32>) {
    let measurements = input.collect::<Vec<i32>>();
    solve1(measurements.windows(3).map(|m| m.iter().sum::<i32>()));
}
