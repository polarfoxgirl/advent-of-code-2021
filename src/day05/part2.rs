use std::fs;
use std::str;
use regex::Regex;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve() {
    let vents = read_input(&String::from("src\\day05\\inputs\\input.txt"));
    println!("Input count: {}", vents.len());

    let result = count_points(&vents);
    println!("Result: {}", result)
}

fn read_input(filename: &String) -> Vec<((i32, i32), (i32, i32))> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    text.split("\r\n").map(|x| parse_line(&re, x)).collect()
}

fn parse_line(re: &Regex, line: &str) -> ((i32, i32), (i32, i32)) {
    let captures = re.captures(line).unwrap();

    let x0 = captures.get(1).unwrap().as_str().parse().unwrap();
    let y0 = captures.get(2).unwrap().as_str().parse().unwrap();

    let x1 = captures.get(3).unwrap().as_str().parse().unwrap();
    let y1 = captures.get(4).unwrap().as_str().parse().unwrap();

    ((x0, y0), (x1, y1))
}

fn count_points(vents: &Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut points_coverage = HashMap::new();

    for ((x0, y0), (x1, y1)) in vents {
        for point in get_points((*x0, *y0), (*x1, *y1)) {
            let coverage = points_coverage.entry(point).or_insert(0);
            *coverage += 1;
        }
    }

   i32::try_from(points_coverage.values().filter(|v| **v > 1).count()).unwrap()
}

fn get_points((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> Vec<(i32, i32)> {
    if x0 == x1 {
        if y0 <= y1 {
            return (y0..(y1 + 1)).map(|y| (x0, y)).collect();
        } else {
            return get_points((x1, y1), (x0, y0));
        } 
    }

    if y0 == y1 {
        if x0 <= x1 {
            return (x0..(x1 + 1)).map(|x| (x, y0)).collect();
        } else {
            return get_points((x1, y1), (x0, y0));
        }
    }

    if x1 - x0 == y1 - y0 {
        if x0 <= x1 {
            return (0..(x1 - x0 + 1)).map(|k| (x0 + k, y0 + k)).collect();
        } else {
            return get_points((x1, y1), (x0, y0));
        }
    }

    if x1 - x0 == y0 - y1 {
        if x0 <= x1 {
            return (0..(x1 - x0 + 1)).map(|k| (x0 + k, y0 - k)).collect();
        } else {
            return get_points((x1, y1), (x0, y0));
        }
    }

    Vec::new()
}