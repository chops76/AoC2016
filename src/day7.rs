use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

type Input = Vec<(Vec<String>, Vec<String>)>;

fn parse_line(s: &str) -> (Vec<String>, Vec<String>) {
	let mut yes_strings = Vec::new();
	let mut no_strings = Vec::new();
	let spl: Vec<&str> = s.split(|c: char| c.is_ascii_punctuation()).collect();
	for i in (0..spl.len()).step_by(2) {
		yes_strings.push(spl[i].to_string());
	}
	for i in (1..spl.len()).step_by(2) {
		no_strings.push(spl[i].to_string());
	}

	(yes_strings, no_strings)
}

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn has_abba(s: &str) -> bool {
	let chars = s.as_bytes();
	for i in 0..s.len() - 3 {
		if chars[i] != chars[i+1] && chars[i] == chars[i+3] && chars[i+1] == chars[i+2] {
			return true;
		}
	}

	false
}

fn part1(sequences: &Input) -> u32 {
	let mut count = 0;
	for seq in sequences {
		let mut valid = false;
		for s in &seq.0 {
			if has_abba(s) {
				valid = true;
				break;
			}
		}
		if !valid {
			continue;
		}
		for s in &seq.1 {
			if has_abba(s) {
				valid = false;
				break;
			}
		}
		if valid {
			count += 1;
		}
	}
	count
}

fn get_babs(sequences: &Vec<String>) -> Vec<String> {
	let mut hs = HashSet::new();
	for s in sequences {
		let v: Vec<char> = s.chars().collect();
		for i in 0..s.len()-2 {
			if v[i] != v[i+1] && v[i] == v[i+2] {
				let bab = vec![v[i+1], v[i], v[i+1]].iter().collect::<String>();
				hs.insert(bab);
			}
		}
	}

	hs.iter().map(|s| s.clone()).collect()
}

fn part2(sequences: &Input) -> u32 {
	let mut count = 0;
	for seq in sequences {
		let babs = get_babs(&seq.0);

		let mut found = false;
		for s in &seq.1 {
			for bab in &babs {
				if s.contains(bab) {
					found = true;
					break;
				}
			}
			if found {
				break;
			}
		}
		if found {
			count += 1;
		}
	}
	count
}

pub fn main() {
	let sequences = parse_input("./input/day7/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&sequences);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&sequences);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}


