use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let crabs = read_input(&String::from("src\\day07\\inputs\\input.txt"));
    println!("Input count: {}", crabs.len());

    crabs.sort();
    let (position, fuel) = calc_best_fuel(&crabs);
    
    println!("Result: {} {}", position, fuel)
}

fn read_input(filename: &String) -> Vec<i32> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split(",").map(|x| x.parse().unwrap()).collect()
}

fn calc_best_fuel(crabs: &Vec<i32>) -> (i32, i32) {
    let mut best_fuel = calc_fuel(&crabs, &crabs[0]);
    for position in (crabs[0] + 1)..(crabs.last().unwrap() + 1) {
        let fuel = calc_fuel(&crabs, &position);
        if fuel < best_fuel {
            best_fuel = fuel;
        } else {
            return (position - 1, best_fuel);
        }
    }

    panic!("Could not find optimal fuel")
}

fn calc_fuel(crabs: &Vec<i32>, position: &i32) -> i32 {
    let mut fuel = 0;
    for crab in crabs {
       fuel += i32::abs(*position - *crab);
    }

    fuel
}