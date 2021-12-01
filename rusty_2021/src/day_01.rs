pub fn solve(input: String) {
    let input = input.lines().map(|m| m.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    solve0(input.clone(), 2);
    solve0(input, 4);
}

fn solve0(input: Vec<usize>, size: usize) {
    println!(
        "Day1: {}",
        input.windows(size).filter(|m| m.last() > m.first()).count()
    )
}
