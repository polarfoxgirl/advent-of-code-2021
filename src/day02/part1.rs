use std::fs;
use regex::Regex;

#[allow(dead_code)]
pub fn solve() {
    let depths = read_input(String::from("src\\day02\\inputs\\input.txt"));
    println!("Input count: {}", depths.len());

    let result = depths.iter().fold((0, 0), |acc, cmd| apply_command(acc, cmd));

    println!("Result: {} {:?}", result.0 * result.1, result)
}

enum SubCommand {
    Forward,
    Up,
    Down,
}

fn read_input(filename: String) -> Vec<(SubCommand, i32)> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    let re = Regex::new(r"(\w+) (\d+)").unwrap();
    text.split("\r\n").map(|x| parse_line(&re, x)).collect()
}

fn parse_line(re: &Regex, line: &str) -> (SubCommand, i32) {
    let captures = re.captures(line).unwrap();

    let direction = match captures.get(1).unwrap().as_str() {
        "forward" => SubCommand::Forward,
        "up" => SubCommand::Up,
        "down" => SubCommand::Down,
        _ => panic!("Unable to parse command")
    };

    let value: i32 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    
    (direction, value)
}

fn apply_command(position: (i32, i32), command: &(SubCommand, i32)) -> (i32, i32) {
    match command.0{
        SubCommand::Forward => (position.0 + command.1, position.1),
        SubCommand::Up => (position.0, position.1 - command.1),
        SubCommand::Down => (position.0, position.1 + command.1),
    }
}