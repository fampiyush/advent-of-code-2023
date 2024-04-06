use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./src/bin/input-07.txt").expect("Error in reading input");

    let v: Vec<hands> = input
        .lines()
        .map(|x| {
            let (hand, bet) = x.split_once(" ").unwrap();
            let kind = get_type(hand);
            hands::new(hand, kind, bet.parse::<usize>().unwrap())
        })
        .collect();

    let mut curr_rank = v.len();
    let mut total = 0;
    for i in (0..=6).rev() {
        let mut curr_hands: Vec<hands> = v
            .iter()
            .filter(|h| h.kind == i)
            .map(|h| h.clone())
            .collect();

        curr_hands.sort_by(|a, b| {
            for (c1, c2) in a.hand.chars().zip(b.hand.chars()) {
                if c1 != c2 {
                    return get_corresponding_value(c2).cmp(&get_corresponding_value(c1));
                } else {
                    continue;
                }
            }
            return std::cmp::Ordering::Equal;
        });

        for h in curr_hands.iter() {
            total += h.bet * curr_rank;
            curr_rank -= 1;
        }
    }

    println!("Total winnings: {}", total);
}

#[derive(Debug, Clone)]
struct hands {
    hand: String,
    kind: usize,
    bet: usize,
}

impl hands {
    fn new(hand: &str, kind: usize, bet: usize) -> Self {
        hands {
            hand: hand.to_string(),
            kind,
            bet,
        }
    }
}

fn get_type(hand: &str) -> usize {
    let mut h: HashMap<char, usize> = HashMap::new();

    for c in hand.chars() {
        h.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    match h.len() {
        1 => return 6,
        2 => {
            if h.iter().any(|v| *v.1 == 4) {
                return 5;
            } else {
                return 4;
            }
        }
        3 => {
            if h.iter().any(|v| *v.1 == 3) {
                return 3;
            } else {
                return 2;
            }
        }
        4 => return 1,
        5 => return 0,
        _ => unreachable!(),
    }
}

fn get_corresponding_value(c: char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as usize,
    }
}
