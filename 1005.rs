use std::io;
use std::vec::Vec;

fn solve(
    weights: &Vec<u32>,
    sum_weights : u32,
    current_select_indices : &Vec<usize>,
    mut global_min_value : u32) {

    let mut sum = 0;
    for i in current_select_indices {
        sum += weights[i];
    }

    let current_min_value = (2 * sum as i32 - sum_weights as i32).abs() as u32;
    
    if current_min_value < global_min_value  {
        global_min_value = current_min_value;
    }

    let weight_count = weights.len();
    let current_selected_count = current_select_indices.len();

    for j in current_selected_count .. 0 {
        let i = current_select_indices[j];
        if i != (weight_count - (current_selected_count - i)) {
            let mut new_select_indices = current_select_indices.to_vec();
            new_select_indices[j] = i + 1;
            solve(weights, sum_weights, new_select_indices, global_min_value);
        }
    }
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    input.clear();
    
    io::stdin().read_line(&mut input).unwrap();

    let weights : Vec<_> = input.trim().split(' ').map(|x| x.parse::<u32>().unwrap()).collect();
    
    println!("{:?}", weights);
}
