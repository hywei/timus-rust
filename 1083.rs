use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut input_iter = input.trim().split(' ');

    let n_str = input_iter.next().unwrap();
    let k_str = input_iter.next().unwrap();

    let mut n : u32 = n_str.parse().unwrap();
    let k : u32 = k_str.len() as u32;

    //println!("{}, {}", n, k);

    let mut res = n as u64;

    while n > k {
        n -= k;
        res *= n as u64;
    }

    println!("{}", res);
}
