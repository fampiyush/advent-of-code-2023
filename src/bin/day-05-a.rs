// Part 1 of day 5
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/input-05.txt").expect("Error reading input file");

    let v: Vec<&str> = input.split("\r\n\r\n").collect();

    let v: Vec<Vec<&str>> = v
        .iter()
        .map(|it| {
            it.split(":")
                .nth(1)
                .unwrap()
                .lines()
                .filter(|&line| !line.trim().is_empty())
                .collect::<Vec<&str>>()
        })
        .collect();

    let mut locations: Vec<usize> = Vec::new();

    v[0][0].split_whitespace().for_each(|seed| {
        locations.push(get_location(
            seed.parse::<usize>().unwrap(),
            &v[1..].to_vec(),
        ))
    });

    let min = locations.iter().min().unwrap();
    println!("Lowest Location: {}", min);
}

fn get_location(seed: usize, v: &Vec<Vec<&str>>) -> usize {
    let mut curr_seed = seed;

    for mapping in v.iter() {
        for li in mapping.iter() {
            let categories: Vec<usize> =
                li.split(" ").map(|d| d.parse::<usize>().unwrap()).collect();
            if (categories[1]..categories[2] + categories[1]).contains(&curr_seed) {
                curr_seed = curr_seed - categories[1] + categories[0];
                break;
            }
        }
    }

    curr_seed
}
