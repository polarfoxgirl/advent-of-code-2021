use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let depths = read_input(String::from("src\\day01\\inputs\\input.txt"));
    println!("Input count: {}", depths.len());    

    println!("Result: {}", count_window_diffs(depths))
}

fn count_window_diffs(depths: Vec<i32>) -> i32 {
    if depths.len() < 3 {
        return 0;
    }

    let mut count = 0;

    let mut sliding_window: i32 = depths[0..3].iter().sum();
    for i in 3..depths.len() {
        let prev_window = sliding_window;
        sliding_window = sliding_window - depths[i - 3] + depths[i];

        if prev_window < sliding_window {
            count = count + 1;
        }
    }

    count
}

fn read_input(filename: String) -> Vec<i32> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split("\r\n").map(|x| x.parse().unwrap()).collect()
}