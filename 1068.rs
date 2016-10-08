use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n : i64 = input.trim().parse().unwrap();

    let res = match n > 0 {
        true => ( n * ( n + 1)) / 2,
        false =>  1 + ( (n * ( 1 - n)) / 2),
    };

    println!("{}", res);
}
