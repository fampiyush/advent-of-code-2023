// Part 1 of day 3
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/input-03.txt").expect("Error reading input");

    let mut symbols: Vec<Symbol> = Vec::new();

    get_symbols(&input, &mut symbols);

    let mut sum = 0;
    input
        .lines()
        .enumerate()
        .for_each(|(i, line)| sum += get_number(line.trim(), i, &symbols));

    dbg!(sum);
}

#[derive(Debug)]
struct PartNumber {
    value: usize,
    x1: usize,
    x2: usize,
    y: usize,
}

impl PartNumber {
    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        if (self.x1.saturating_sub(1)..=self.x2 + 1).contains(&symbol.x)
            && (self.y.saturating_sub(1)..=self.y + 1).contains(&symbol.y)
        {
            return true;
        }
        false
    }
}

#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
}

fn get_symbols(lines: &str, symbols: &mut Vec<Symbol>) {
    lines.lines().enumerate().for_each(|(iy, line)| {
        line.trim().chars().enumerate().for_each(|(ix, c)| {
            if !c.is_digit(10) && c != '.' {
                symbols.push(Symbol { x: ix, y: iy })
            }
        })
    });
}

fn get_number(line: &str, iy: usize, symbols: &Vec<Symbol>) -> usize {
    let mut numbers = Vec::new();
    let mut curr = String::new();
    let mut x1 = 0;
    for (index, n) in line.chars().enumerate() {
        if n.is_digit(10) {
            if curr.is_empty() {
                x1 = index;
            }
            curr.push(n);
            continue;
        }

        if !curr.is_empty() {
            numbers.push(PartNumber {
                x1,
                x2: index - 1,
                value: curr.parse::<usize>().unwrap(),
                y: iy,
            });
            curr = String::new();
        }
    }
    if !curr.is_empty() {
        numbers.push(PartNumber {
            x1,
            x2: line.len() - 1,
            value: curr.parse::<usize>().unwrap(),
            y: iy,
        });
    }

    let mut sum = 0;
    for number in numbers {
        for symbol in symbols {
            if number.is_adjacent(symbol) {
                // dbg!(&number.value);
                sum += number.value;
            }
        }
    }
    sum
}
