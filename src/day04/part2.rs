use std::fs;
use std::str;
use std::iter;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn solve() {
    let (draws, boards) = read_input(&String::from("src\\day04\\inputs\\input.txt"));
    println!("Input count: {} draws, {} boards", draws.len(), boards.len());

    let (draw, score) = find_looser(&draws, &boards);
    println!("Result: {} * {} = {}", draw, score, draw * score)
}

fn read_input(filename: &String) -> (Vec<i32>, Vec<[[i32; 5]; 5]>) {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    let mut blocks : str::Split<&str> = text.split("\r\n\r\n");

    let draws: Vec<i32> = blocks.next().unwrap().split(",").map(|x| x.parse().unwrap()).collect();
    let boards: Vec<[[i32; 5]; 5]> = blocks.map(|x| parse_board(x)).collect();

    (draws, boards)
}

fn parse_board(text: &str) -> [[i32; 5]; 5] {
    let mut board = [[0; 5]; 5];

    for (i, row) in text.split("\r\n").enumerate() {
        if i >= 5 {
            panic!("Invalid board height");
        }

        for (j, value) in row.split_whitespace().enumerate() {
            if i >= 5 {
                panic!("Invalid board width");
            }

            board[i][j] = value.parse().unwrap();
        }
    }

    board
}

fn find_looser(draws: &Vec<i32>, boards: &Vec<[[i32; 5]; 5]>) -> (i32, i32) {
    let mut marks: Vec<[[bool; 5]; 5]> = iter::repeat([[false; 5]; 5]).take(boards.len()).collect();
    let mut winners: HashSet<usize> = HashSet::new();
    let mut last_winner = None;

    for draw in draws {
        for n in 0..boards.len() {
            if winners.contains(&n) {
                continue;
            }

            let is_winner = mark_on_board(draw, &boards[n], &mut marks[n]);

            if is_winner {
                winners.insert(n);
                last_winner = Some((n, draw))
            }
        }
    }

    if last_winner.is_some() {
        let (n, draw) = last_winner.unwrap();
        (*draw, calc_score(&boards[n], &marks[n]))
    } else {
        panic!("No last winners found")
    }
}

fn mark_on_board(draw: &i32, board: &[[i32; 5]; 5], marks: &mut [[bool; 5]; 5]) -> bool {
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j] == *draw {
                marks[i][j] = true;
                if marks[i].iter().all(|x| *x) {
                    return true;
                }

                if marks.iter().all(|row| row[j]) {
                    return true;
                }
            }
        }
    }

    false
}

fn calc_score(board: &[[i32; 5]; 5], marks: &[[bool; 5]; 5]) -> i32 {
    let mut score = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !marks[i][j] {
                score = score + board[i][j];
            }
        }
    }
    score
}