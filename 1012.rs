extern crate num;

use std::io;
use num::{BigInt};

const MAX_N : usize = 17;
const MAX_K : usize = 11;

fn solve(mut f: &mut [[i32; MAX_K]; MAX_N], n : usize, k : usize) -> i32 {

   // println!("{}, {}", n, k);

    if f[n][k] == -1 {
        let part1 : i32 = solve(&mut f, n-1, k);
        let part2 : i32 = solve(&mut f, n-2, k);

        f[n][k] = (k as i32 - 1) * (part1 + part2) as i32
    }

    return f[n][k];
}

fn main() {
    let mut n_str = String::new();
    let mut k_str = String::new();

    io::stdin().read_line(&mut n_str).unwrap();
    io::stdin().read_line(&mut k_str).unwrap();

    let n = n_str.trim().parse::<u32>().unwrap();
    let k = k_str.trim().parse::<u32>().unwrap();

    let mut f : [[BigInt; MAX_K]; MAX_N] = [[-1; MAX_K]; MAX_N];

    for k_i in 0..MAX_K {
        f[0][k_i] = 1;
        f[1][k_i] = k as i32 - 1;
    }

    for n_i in 0..MAX_N {
        f[n_i][0] = 0;
    }

    let res = solve(&mut f, n as usize, k as usize);

    println!("{}", res);
}
