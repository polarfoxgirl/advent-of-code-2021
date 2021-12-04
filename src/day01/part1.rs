use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let depths = read_input(String::from("src\\day01\\inputs\\input.txt"));
    println!("Input count: {}", depths.len());

    let mut count = 0;
    for i in 0..(depths.len() - 1) {
        if depths[i] < depths[i+1] {
            count = count + 1;
        }        
    }

    println!("Result: {}", count)
}

fn read_input(filename: String) -> Vec<i32> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split("\r\n").map(|x| x.parse().unwrap()).collect()
}