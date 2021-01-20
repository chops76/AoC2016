use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;
use itertools::Itertools;

type Input = Vec<String>;

fn parse_input(path: &str) -> Input {
    BufReader::new(File::open(path).unwrap()).lines().flatten().collect()
}

fn part1(commands: &Input, word: &VecDeque<char>) -> String {
    let mut letters = word.clone();

    for c in commands {
        let tokens = c.split(" ").collect::<Vec<&str>>();
        match tokens[0] {
            "swap" => {
                if tokens[1] == "position" {
                    let a = tokens[2].parse().unwrap();
                    let b = tokens[5].parse().unwrap();
                    let tmp = letters[a];
                    letters[a] = letters[b];
                    letters[b] = tmp;
                } else {
                    let a = letters.iter().position(|&c| c == tokens[2].chars().next().unwrap()).unwrap(); 
                    let b = letters.iter().position(|&c| c == tokens[5].chars().next().unwrap()).unwrap(); 
                    let tmp = letters[a];
                    letters[a] = letters[b];
                    letters[b] = tmp;
                }
            },
            "reverse" => {
                let a = tokens[2].parse().unwrap();
                let b = tokens[4].parse().unwrap();   
                letters.make_contiguous()[a..=b].reverse();             
            },
            "rotate" => {
                if tokens[1] == "left" {
                    let a = tokens[2].parse().unwrap();
                    letters.make_contiguous().rotate_left(a);
                } else if tokens[1] == "right" {
                    let a = tokens[2].parse().unwrap();
                    letters.make_contiguous().rotate_right(a);
                } else {
                    let idx = letters.iter().position(|&c| c == tokens[6].chars().next().unwrap()).unwrap();
                    let mut rot_num = idx + 1;
                    if idx >= 4 {
                        rot_num += 1;
                    }
                    rot_num %= letters.len();
                    letters.make_contiguous().rotate_right(rot_num);
                }
            },
            "move" => {
                let a = tokens[2].parse().unwrap();
                let b = tokens[5].parse().unwrap();
                let c = letters.remove(a).unwrap();
                letters.insert(b, c); 
            }
            _ => {
                println!("Unknown Command");
            }
        }
    }

    letters.iter().collect()
}

fn part2(commands: &Input, word: &VecDeque<char>) -> String {
    for i in word.iter().permutations(word.len()) {
        if part1(commands, &i.iter().map(|c| **c).collect()) == "fbgdceah" {
            return i.iter().map(|c| **c).collect();
        }
    }
    "Not Found!".to_string()
}

pub fn main() {
    let commands = parse_input("./input/day21/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&commands, &"abcdefgh".chars().collect::<VecDeque<char>>());
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&commands, &"abcdefgh".chars().collect::<VecDeque<char>>());
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

