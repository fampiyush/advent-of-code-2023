use std::fs;

fn main() {
    //getting input from input-02.txt
    let input = fs::read_to_string("./src/bin/input-02.txt").expect("Failed to read input.txt");

    let mut sum: usize = 0;

    //for every line possible function is called. Change second argument of possible for 1st part
    input
        .lines()
        .for_each(|line| sum += possible(line.trim(), 2));

    println!("Sum of IDs: {}", sum);
}

fn possible(line: &str, part: usize) -> usize {
    let mut split = line.split(":");
    let game_num = split.next().unwrap();
    let games = split.next().unwrap();

    let game_num = game_num
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let games: Vec<&str> = games.split(|c| c == ';' || c == ',').collect();

    match part {
        1 => {
            for game in games {
                let mut split = game.split_whitespace();
                let value = split.next().unwrap().parse::<usize>().unwrap();
                let color = split.next().unwrap();

                match color {
                    "red" if value > 12 => return 0,
                    "green" if value > 13 => return 0,
                    "blue" if value > 14 => return 0,
                    _ => (),
                }
            }
            game_num
        }
        2 => {
            let (mut red, mut blue, mut green) = (0, 0, 0);
            for game in games {
                let mut split = game.split_whitespace();
                let value = split.next().unwrap().parse::<usize>().unwrap();
                let color = split.next().unwrap();

                match color {
                    "red" if value > red => red = value,
                    "green" if value > green => green = value,
                    "blue" if value > blue => blue = value,
                    _ => (),
                }
            }
            red * green * blue
        }
        _ => 0,
    }
}
