use std::fs;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn solve() {
    let lines = read_input(&String::from("src\\day08\\inputs\\test.txt"));
    println!("Input count: {}", lines.len());
    
    resolve_line(&lines[0]);
    // println!("Result: {}", result)
}

fn read_input(filename: &String) -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split("\r\n").map(|x| parse_line(x)).collect()
}

fn parse_line(line: &str) -> (Vec<HashSet<char>>, Vec<HashSet<char>>) {
    let parts : Vec<&str> = line.split('|').collect();
    if parts.len() != 2 {
        panic!("Invalid input");
    }

    let displays = parts[0]
        .split_whitespace()
        .map(|x| HashSet::from_iter(x.chars()))
        .collect();
    let digits = parts[1]
        .split_whitespace()
        .map(|x| HashSet::from_iter(x.chars()))
        .collect();

    (displays, digits)
}

fn resolve_line((displays, digits): &(Vec<HashSet<char>>, Vec<HashSet<char>>)) -> i32 {
    let all_inputs : Vec<&HashSet<char>> = displays.iter().chain(digits.iter()).collect();

    let starting_rules : Vec<(&HashSet<char>, HashSet<Vec<char>>)> = all_inputs.iter()
        .filter(|x| (**x).len() < 7)
        .map(|x| (*x, process_digit(*x)))
        .collect();

    println!("{:?}", starting_rules);

    0
}

fn process_digit(input: &HashSet<char>) -> HashSet<Vec<char>> {
    match input.len() {
        2 => HashSet::from([vec!['c'], vec!['f']]),
        3 => HashSet::from([vec!['a'], vec!['c'], vec!['f']]),
        4 => HashSet::from([vec!['b'], vec!['c'], vec!['d'], vec!['f']]),
        5 => HashSet::from([vec!['a'], vec!['c'], vec!['d'], vec!['e', 'f'], vec!['f']]),
        6 => HashSet::from([vec!['a'], vec!['b'], vec!['c', 'e', 'd', 'f'], vec!['g']]),
        _ => panic!("Unexpected input length {}", input.len())
    }
}

fn get_all_rules(starting_rules: &Vec<(&HashSet<char>, HashSet<Vec<char>>)>) {
    let mut all_rules = HashSet::new();

    // TODO: get all the intersections
}
