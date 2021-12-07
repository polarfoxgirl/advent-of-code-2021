use std::fs;
use std::iter;

#[allow(dead_code)]
pub fn solve() {
    let bin_nums = read_input(&String::from("src\\day03\\inputs\\input.txt"));
    println!("Input count: {}", bin_nums.len());

    let bin_len = bin_nums.first().unwrap().len();
    let init_counters = iter::repeat((0, 0)).take(bin_len).collect();

    let result_counters = bin_nums.iter().fold(init_counters, count_bin_num);
    let gamma_epsilon = calc_gamma_epsilon(&result_counters);

    println!("Result: {} {:?}", gamma_epsilon.0 * gamma_epsilon.1, gamma_epsilon)
}

fn read_input(filename: &String) -> Vec<Vec<char>> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split("\r\n").map(|x| x.chars().collect()).collect()
}

fn count_bin_num(counters: Vec<(i32, i32)>, bin_num: &Vec<char>) -> Vec<(i32, i32)> {
    if counters.len() != bin_num.len() {
        panic!("Inconsistent line lengths")
    }

    let pair_iter = counters.iter().zip(bin_num.iter());

    pair_iter.map(|x| count_bit(x.0, x.1)).collect()
}

fn count_bit(counter: &(i32, i32), bit: &char) -> (i32, i32) {
    match bit {
        '0' => (counter.0 + 1, counter.1),
        '1' => (counter.0, counter.1 + 1),
        _ => panic!("Invalid char")
    }
}

fn calc_gamma_epsilon(counters: &Vec<(i32, i32)>) -> (i32, i32) {
    let mut gamma = 0;
    let mut epsilon = 0;

    for bit_counter in counters {
        if bit_counter.0 > bit_counter.1 {
            epsilon = epsilon + 1;
        } else {
            gamma = gamma + 1;
        }

        gamma = gamma << 1;
        epsilon = epsilon << 1;
    }

    (gamma >> 1, epsilon >> 1)
}