use std::fs;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn solve() {
    let lines = read_input(&String::from("src\\day08\\inputs\\input.txt"));
    println!("Input count: {}", lines.len());

    let easy_lengths = HashSet::from([2, 3, 4, 7]);
    let result: usize = lines.iter().map(|x| count_easy_digits(&easy_lengths, x)).sum();
    
    println!("Result: {}", result)
}

fn read_input(filename: &String) -> Vec<(Vec<String>, Vec<String>)> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split("\r\n").map(|x| parse_line(x)).collect()
}

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
    let parts : Vec<&str> = line.split('|').collect();
    if parts.len() != 2 {
        panic!("Invalid input");
    }

    let displays = parts[0].split_whitespace().map(|x| x.to_owned()).collect();
    let digits = parts[1].split_whitespace().map(|x| x.to_owned()).collect();

    (displays, digits)
}

fn count_easy_digits(easy_lengths: &HashSet<usize>, (_, digits): &(Vec<String>, Vec<String>)) -> usize {
    digits.iter().filter(|d| easy_lengths.contains(&d.len())).count()
}