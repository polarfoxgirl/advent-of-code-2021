use std::fs;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn solve() {
    let bin_nums = read_input(&String::from("src\\day03\\inputs\\input.txt"));
    println!("Input count: {}", bin_nums.len());

    let o2_pos = filter_based_on_pos(
        &bin_nums,
        filter_most_common,
        0,
        &HashSet::from_iter(0..bin_nums.len()));
    let o2_generator_rating = bin_num_to_dec(&bin_nums[o2_pos]);

    let co2_pos = filter_based_on_pos(
        &bin_nums,
        filter_least_common,
        0,
        &HashSet::from_iter(0..bin_nums.len()));
    let co2_scrubber_rating = bin_num_to_dec(&bin_nums[co2_pos]);

    println!("Result: {} {} {}", o2_generator_rating, co2_scrubber_rating, o2_generator_rating * co2_scrubber_rating)
}

fn read_input(filename: &String) -> Vec<Vec<bool>> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split("\r\n").map(|x| x.chars().map(parse_bit).collect()).collect()
}

fn parse_bit(bit: char) -> bool {
    match bit {
        '0' => false,
        '1' => true,
        _ => panic!("Invalid char")
    }
}

fn filter_based_on_pos<F>(bin_nums: &Vec<Vec<bool>>, filter: F, bin_pos: usize, indices: &HashSet<usize>) -> usize 
    where F : Fn(&Vec<Vec<bool>>, &usize, &HashSet<usize>) -> bool{
    if bin_pos > bin_nums[0].len() {
        panic!("Failed to limit search to a single number")
    }

    let filter_value = filter(bin_nums, &bin_pos, indices);
    let mut filtered_inidices = HashSet::new();

    for i in indices {
        if bin_nums[*i][bin_pos] == filter_value {
            filtered_inidices.insert(*i);
        }
    }

    if filtered_inidices.len() == 1 {
        *filtered_inidices.iter().next().unwrap()
    } else {
        filter_based_on_pos(bin_nums, filter, bin_pos + 1, &filtered_inidices)
    }
}

fn filter_most_common(bin_nums: &Vec<Vec<bool>>, bin_pos: &usize, indices: &HashSet<usize>) -> bool {
    let mut one_counter = 0; 
    for i in indices {
        if bin_nums[*i][*bin_pos] {
            one_counter = one_counter + 1;
        }
    }

    2 * one_counter >= indices.len()
}

fn filter_least_common(bin_nums: &Vec<Vec<bool>>, bin_pos: &usize, indices: &HashSet<usize>) -> bool {
    let mut one_counter = 0; 
    for i in indices {
        if bin_nums[*i][*bin_pos] {
            one_counter = one_counter + 1;
        }
    }

    2 * one_counter < indices.len()
}

fn bin_num_to_dec(bin_num: &Vec<bool>) -> i32 {
    let mut result = 0;

    for bit in bin_num {
        if *bit {
            result = result + 1;
        }
        result = result << 1;
    }

    result >> 1
}