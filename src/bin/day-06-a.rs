//Part 1 of day 6

use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/input-06.txt").expect("Error reading input");

    let v: Vec<&str> = input.lines().collect();
    let time: Vec<usize> = v[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distance: Vec<usize> = v[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut total_ways: usize = 1;

    time.iter()
        .enumerate()
        .for_each(|(i, t)| total_ways *= ways(&t, &distance[i]));

    println!("total ways: {total_ways}");
}

fn ways(time: &usize, distance: &usize) -> usize {
    let mut ways: usize = 0;

    for i in 1..(*time) {
        if ((*time) - i) * i > (*distance) {
            ways += 1;
        }
    }

    ways
}
