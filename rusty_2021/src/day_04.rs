use crate::lib::Solve;

pub struct Day4 {}

type Score = Vec<usize>;
type Board = Vec<Vec<Token>>;
type Token = Option<usize>;

fn parse(input: String) -> (Score, Vec<Board>) {
    let (score, boards) = input.split_once("\n\n").unwrap();

    let score = score.split(',').map(|x| x.parse().unwrap()).collect();
    let boards = boards
        .replace("\n\n", "\r")
        .split("\r")
        .map(|board| {
            let board: Board = board
                .split("\n")
                .map(|line| {
                    line.split(' ')
                        .map(|token| token.parse().ok())
                        .filter(|token| token.is_some())
                        .collect::<Vec<Token>>()
                })
                .collect::<Board>();
            board
        })
        .collect::<Vec<Board>>();

    (score, boards)
}

fn mark_boards(boards: &mut Vec<Board>, score: usize) {
    let nb_boards = boards.len();
    let nb_row = boards[0].len();
    let nb_col = nb_row;
    for i in 0..nb_boards {
        for row in 0..nb_row {
            for col in 0..nb_col {
                if boards[i][row][col] == Some(score) {
                    boards[i][row][col] = None;
                }
            }
        }
    }
}

pub fn check_boards(boards: &Vec<Board>) -> Option<usize> {
    let nb_boards = boards.len();
    let nb_row = boards[0].len();
    let nb_col = nb_row;
    let mut mark_row;
    let mut mark_col;
    for i in 0..nb_boards {
        // Check row
        for row in 0..nb_row {
            mark_row = 0;
            for token in &boards[i][row] {
                if token.is_none() {
                    mark_row += 1;
                } else {
                    break;
                }
                if mark_row == 5 {
                    return Some(i);
                }
            }
        }

        //Check column
        for col in 0..nb_col {
            mark_col = 0;
            for row in 0..nb_row {
                if boards[i][row][col].is_none() {
                    mark_col += 1;
                } else {
                    break;
                }
                if mark_col == 5 {
                    return Some(i);
                }
            }
        }
    }
    None
}

pub fn solve(input: String) -> String {
    let (scores, mut boards) = parse(input);
    let mut prev_score = 0;
    let mut winning_board: Option<usize> = None;
    for i in 0..scores.len() {
        prev_score = scores[i];
        mark_boards(&mut boards, scores[i]);
        winning_board = check_boards(&boards);
        if winning_board.is_some() {
            break;
        }
    }

    if winning_board.is_none() {
        println!("No solutions");
        std::process::exit(1);
    }

    let board_sum: usize = boards[winning_board.unwrap()]
        .iter()
        .map(|line| line.iter().map(|token| token.unwrap_or(0)).sum::<usize>())
        .sum();
    let answer = prev_score * board_sum;
    answer.to_string()
}

pub fn solve2(input: String) -> String {
    let (scores, mut boards) = parse(input);
    let mut winning_boards: Vec<(Board, usize)> = Vec::new();
    for score in scores {
        if boards.is_empty() {
            break;
        }
        mark_boards(&mut boards, score);
        let won_index = check_boards2(&boards);
        for index in won_index.clone() {
            winning_boards.push((boards[index].clone(), score));
        }

        boards = boards
            .iter()
            .enumerate()
            .filter(|(index, _)| won_index.iter().find(|i| *i == index).is_none())
            .map(|(_, b)| b)
            .cloned()
            .collect::<Vec<Board>>();
    }

    let (winning_board, score) = winning_boards.pop().unwrap();
    let board_sum: usize = winning_board
        .iter()
        .map(|line| line.iter().map(|token| token.unwrap_or(0)).sum::<usize>())
        .sum();
    let answer = board_sum * score;
    answer.to_string()
}

pub fn check_boards2(boards: &Vec<Board>) -> Vec<usize> {
    let nb_boards = boards.len();
    let nb_row = boards[0].len();
    let nb_col = nb_row;
    let mut mark_row;
    let mut mark_col;
    let mut won_index = Vec::new();
    for i in 0..nb_boards {
        // Check row
        for row in 0..nb_row {
            mark_row = 0;
            for token in &boards[i][row] {
                if token.is_none() {
                    mark_row += 1;
                } else {
                    break;
                }
                if mark_row == 5 {
                    println!("PUSH");
                    won_index.push(i);
                }
            }
        }

        //Check column
        for col in 0..nb_col {
            mark_col = 0;
            for row in 0..nb_row {
                if boards[i][row][col].is_none() {
                    mark_col += 1;
                } else {
                    break;
                }
                if mark_col == 5 {
                    println!("szxfdcgjvhjb");
                    won_index.push(i);
                }
            }
        }
    }
    won_index
}

impl Solve for Day4 {
    fn name() -> String {
        "Sonar Sweep".into()
    }

    fn part_one(input: String) -> String {
        solve(input)
    }

    fn part_two(input: String) -> String {
        solve2(input)
    }
}
