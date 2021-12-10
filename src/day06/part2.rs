use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let fish_states = read_input(&String::from("src\\day06\\inputs\\input.txt"));
    println!("Input count: {}", fish_states.len());

    let mut state_counts = vec![0i64; 9];
    count_init_states(&fish_states, &mut state_counts);

    for _ in 0..256 {
        apply_day_step(&mut state_counts);
    }

    let result : i64 = state_counts.iter().sum();
    println!("Result: {}", result)
}

fn read_input(filename: &String) -> Vec<usize> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split(",").map(|x| x.parse().unwrap()).collect()
}

fn count_init_states(fish_states: &Vec<usize>, state_count: &mut Vec<i64>) {
    for state in fish_states {
        if *state > 8 {
            panic!("Invalid initial fish state {}", state);
        }

        state_count[*state] += 1;
    }
}

fn apply_day_step(state_counts: &mut Vec<i64>) {
    // Remove all fish with 0 state so every state effectively shifts
    let reproduction_count = state_counts.remove(0);

    // Append newly created fish in the end, effectively to the 8 state
    state_counts.push(reproduction_count);

    // Reset counter for all newly reproduced fish to 6
    state_counts[6] += reproduction_count;
}