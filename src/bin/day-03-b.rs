// Part 2 of day 3
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/input-03.txt").expect("Error reading input");

    let mut symbols: Vec<Symbol> = Vec::new();

    get_symbols(&input, &mut symbols);

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    input
        .lines()
        .enumerate()
        .for_each(|(i, line)| get_number(line.trim(), i, &mut part_numbers));

    let mut gears_sum = 0;
    symbols.iter().for_each(|symbol| {
        if symbol.star == true {
            gears_sum += get_gears(&symbol, &part_numbers)
        }
    });

    println!("Sum: {gears_sum}");
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
    star: bool,
}

fn get_symbols(lines: &str, symbols: &mut Vec<Symbol>) {
    lines.lines().enumerate().for_each(|(iy, line)| {
        line.trim().chars().enumerate().for_each(|(ix, c)| {
            if !c.is_digit(10) && c != '.' {
                symbols.push(Symbol {
                    x: ix,
                    y: iy,
                    star: if c == '*' { true } else { false },
                });
            }
        });
    });
}

fn get_number(line: &str, iy: usize, part_numbers: &mut Vec<PartNumber>) {
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
            part_numbers.push(PartNumber {
                x1,
                x2: index - 1,
                value: curr.parse::<usize>().unwrap(),
                y: iy,
            });
            curr = String::new();
        }
    }
    if !curr.is_empty() {
        part_numbers.push(PartNumber {
            x1,
            x2: line.len() - 1,
            value: curr.parse::<usize>().unwrap(),
            y: iy,
        });
    }
}

fn get_gears(symbol: &Symbol, numbers: &Vec<PartNumber>) -> usize {
    let mut n = 0;
    let mut gear_ratio = 1;
    numbers.iter().for_each(|num| {
        if num.is_adjacent(&symbol) {
            n += 1;
            gear_ratio *= num.value;
        }
    });

    if n == 2 {
        return gear_ratio;
    } else {
        0
    }
}
