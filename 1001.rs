use std::io;
use std::io::prelude::*;
use std::vec::Vec;

fn main() {

    let mut inputs = Vec::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let numbers_str = line.unwrap();
        /*
        if numbers_str == "end" {
            break;
        }
        */
        for number_str in numbers_str.trim().split(' ') {
            let real_str = number_str.trim();
            if real_str.len() > 0 {
                inputs.push( real_str.parse::<u64>().unwrap());
            }
        }

    }

    inputs.reverse();
    for number in inputs {
        let float_number = number as f64;
        println!("{:.4}", float_number.sqrt());
    }
}
