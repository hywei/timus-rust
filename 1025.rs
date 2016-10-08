use std::io;

fn main() {
    let mut half_persons = Vec::new();

    let mut group_count_str = String::new();

    io::stdin().read_line(&mut group_count_str).unwrap();

    let group_count : u32 = group_count_str.trim().parse().unwrap();

    let mut persons_in_group_str = String::new();

    io::stdin().read_line(&mut persons_in_group_str).unwrap();

    for person_in_group_str in persons_in_group_str.trim().split(' ') {
        half_persons.push( (person_in_group_str.parse::<u32>().unwrap() / 2) + 1);
    }

    half_persons.sort();

    let mut count : u32 = 0;

    for p in &half_persons[0 .. (half_persons.len()/2 + 1)] {
        //println!("{}", p);
        count += *p;
    }

    println!("{}", count);
}
