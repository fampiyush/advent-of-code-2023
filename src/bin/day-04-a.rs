// Part 1 of day 4
use ::std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/input-04.txt").expect("Cannot read input file");

    let mut sum = 0;

    input
        .lines()
        .for_each(|line| sum += get_points(line.trim()));

    println!("Sum: {}", sum);
}

fn get_points(line: &str) -> usize {
    let v: Vec<&str> = line.split(":").nth(1).unwrap().trim().split("|").collect();
    let win_num: Vec<usize> = v[0]
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let own_num: Vec<usize> = v[1]
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut points = 0;
    for w_num in win_num {
        if own_num.contains(&w_num) {
            if points != 0 {
                points *= 2;
            } else {
                points = 1;
            }
        }
    }
    points
}
