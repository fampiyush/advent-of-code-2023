//Part 2 of day 6

use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/input-06.txt").expect("Error reading input");

    let v: Vec<&str> = input.lines().collect();
    let mut time = String::new();
    v[0].split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .for_each(|d| time.push_str(d));

    let mut distance = String::new();
    v[1].split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .for_each(|d| distance.push_str(d));

    println!(
        "{}",
        ways(
            &time.parse::<usize>().unwrap(),
            &distance.parse::<usize>().unwrap()
        )
    );
}

fn ways(time: &usize, distance: &usize) -> usize {
    let mut starting_win = 0;

    for i in 1..(*time) {
        if ((*time) - i) * i > (*distance) {
            starting_win = i;
            break;
        }
    }

    time - (2 * starting_win) + 1
}
