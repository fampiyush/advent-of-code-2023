use std::fs;

fn main() {
    // getting the input from input.txt file
    let input = fs::read_to_string("./src/bin/input-01.txt").expect("Failed to read input.txt");

    // change the second argument to part 1 or 2
    get_lines(&input, 1);
}

// splitting the lines and calling get number fn on each line
fn get_lines(input: &str, part: usize) {
    let mut sum: u32 = 0;
    input.lines().for_each(|line| match part {
        1 => sum += get_number(line.trim()),
        2 => sum += get_number_with_letter(line.trim()),
        _ => (),
    });

    println!("Sum: {}", sum);
}

// for part 1
fn get_number(line: &str) -> u32 {
    let mut v: Vec<u32> = Vec::new();
    for i in line.chars() {
        if i.is_digit(10) {
            v.push(i.to_digit(10).unwrap());
        }
    }
    v[0] * 10 + v[v.len() - 1]
}

//for part 2
fn get_number_with_letter(line: &str) -> u32 {
    let letters = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut v: Vec<u32> = Vec::new();
    for (index, ch) in line.chars().enumerate() {
        if ch.is_digit(10) {
            v.push(ch.to_digit(10).unwrap());
        } else {
            for (i, letter) in letters.iter().enumerate() {
                if line[index..].starts_with(letter) {
                    v.push(i as u32 + 1);
                    break;
                }
            }
        }
    }
    // Takes first and last element and make it double digit
    v[0] * 10 + v[v.len() - 1]
}
