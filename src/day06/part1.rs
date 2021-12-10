use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let mut fish_states = read_input(&String::from("src\\day06\\inputs\\input.txt"));
    println!("Input count: {}", fish_states.len());

    for _ in 0..80 {
        apply_day_step(&mut fish_states);
    }

    println!("Result: {}", fish_states.len())
}

fn read_input(filename: &String) -> Vec<i8> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split(",").map(|x| x.parse().unwrap()).collect()
}

fn apply_day_step(fish_states: &mut Vec<i8>) {
    let new_fish_count = fish_states.iter().filter(|x| **x == 0).count();

    decrement_states(fish_states);
    let mut new_fish =  vec![8i8; new_fish_count];
    fish_states.append(&mut new_fish);
}

fn decrement_states(fish_states: &mut Vec<i8>) {
    for state in fish_states {
        if *state == 0 {
            *state = 6;
        } else {
            *state -= 1;
        }
    }
}