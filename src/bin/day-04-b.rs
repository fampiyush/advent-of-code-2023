// Part 2 of day 4
use ::std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/input-04.txt").expect("Cannot read input file");

    let mut total: Vec<usize> = vec![1; input.lines().count()];

    input
        .lines()
        .enumerate()
        .for_each(|(i, line)| get_cards(line.trim(), i + 1, &mut total));

    println!("Total scratchcards: {}", total.iter().sum::<usize>());
}

fn get_cards(line: &str, card_num: usize, total: &mut Vec<usize>) {
    let v: Vec<&str> = line.split(":").nth(1).unwrap().trim().split("|").collect();
    let win_num: Vec<usize> = v[0]
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let own_num: Vec<usize> = v[1]
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut card_to_inc = card_num;
    for w_num in win_num {
        if card_to_inc == total.len() {
            return;
        }
        if own_num.contains(&w_num) {
            total[card_to_inc] += total[card_num - 1];
            card_to_inc += 1;
        }
    }
}
