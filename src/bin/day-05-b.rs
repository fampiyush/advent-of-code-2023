// Part 2 of day 5

// Wrong Solution, will look into it later.

// use ::std::cmp::{max, min};
// use ::std::fs;

// fn main() {
//     let input = fs::read_to_string("./src/bin/input-05.txt").expect("Error reading input file");

//     let v: Vec<&str> = input.split("\r\n\r\n").collect();

//     let v: Vec<Vec<&str>> = v
//         .iter()
//         .map(|it| {
//             it.split(":")
//                 .nth(1)
//                 .unwrap()
//                 .lines()
//                 .filter(|&line| !line.trim().is_empty())
//                 .collect::<Vec<&str>>()
//         })
//         .collect();

//     let mut locations: Vec<usize> = Vec::new();

//     v[0][0]
//         .split_whitespace()
//         .collect::<Vec<&str>>()
//         .chunks(2)
//         .for_each(|pair| {
//             let seed1 = pair[0].parse::<usize>().unwrap();
//             let seed2 = pair[1].parse::<usize>().unwrap();
//             locations.push(get_location((seed1, seed1 + seed2), &v[1..].to_vec()));
//         });

//     let min = locations.iter().min().unwrap();
//     println!("Lowest Location: {}", min);
// }

// fn get_location(seed: (usize, usize), v: &Vec<Vec<&str>>) -> usize {
//     let mut curr_seed: Vec<(usize, usize)> = vec![seed];

//     for mapping in v.iter() {
//         let mut new_seed: Vec<(usize, usize)> = Vec::new();
//         for (seed1, seed2) in curr_seed.iter_mut() {
//             for li in mapping.iter() {
//                 let c: Vec<usize> = li.split(" ").map(|d| d.parse::<usize>().unwrap()).collect();
//                 let st = max(c[1], *seed1);
//                 let end = min(c[2] + c[1], *seed2);

//                 if st < end {
//                     new_seed.push((st - c[1] + c[0], end - c[1] + c[0]));

//                     if *seed1 < st {
//                         curr_seed.push((*seed1, st));
//                     }
//                     if end > *seed2 {
//                         curr_seed.push((end, *seed2));
//                     }
//                     break;
//                 }
//             }
//             new_seed.push((*seed1, *seed2));
//         }
//         curr_seed = new_seed;
//     }

//     curr_seed.sort_by_key(|s| s.0);
//     dbg!(&curr_seed);
//     curr_seed[0].0
// }
