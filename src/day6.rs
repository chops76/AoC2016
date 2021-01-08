use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::char;

type Input = Vec<String>;

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn part1(messages: &Input) -> String {
	let mut work_vec = Vec::new();
	for i in 0..messages[0].len() {
		let mut v = vec![0;26];
		for m in messages {
			v[(m.as_bytes()[i]-97) as usize] += 1;
		}
		work_vec.push((v.iter().enumerate().max_by_key(|(_idx, &val)| val)
		                .map(|(idx, _val)| idx).unwrap() + 97) as u8 as char);
	}
	work_vec.iter().collect()
}

fn part2(messages: &Input) -> String {
	let mut work_vec = Vec::new();
	for i in 0..messages[0].len() {
		let mut v = vec![0;26];
		for m in messages {
			v[(m.as_bytes()[i]-97) as usize] += 1;
		}
		work_vec.push((v.iter().enumerate().filter(|letter| *letter.1 != 0).min_by_key(|(_idx, &val)| val)
		                .map(|(idx, _val)| idx).unwrap() + 97) as u8 as char);
	}
	work_vec.iter().collect()
}

pub fn main() {
	let messages = parse_input("./input/day6/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&messages);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&messages);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day6_test1() {
		let messages = parse_input("./input/day6/test.txt");
		assert_eq!(part1(&messages), "easter");
	}

	#[test]
	fn day6_test2() {
		let messages = parse_input("./input/day6/test.txt");
		assert_eq!(part2(&messages), "advent");
	}
}

