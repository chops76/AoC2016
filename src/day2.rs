use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::char;

type Input = Vec<String>;

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn part1(dirs: &Input) -> String {
	let mut cv = Vec::new();
	let mut num = 5;
	for digit in dirs {
		for dir in digit.chars() {
			match dir {
				'U' => {
					if num > 3 {
						num -= 3;
					}
				},
				'D' => {
					if num < 7 {
						num += 3;
					}
				},
				'R' => {
					if num % 3 != 0 {
						num += 1;
					}
				},
				_ => {
					if num % 3 != 1 {
						num -= 1;
					}
				}
			}
		}
		cv.push(char::from_digit(num, 10).unwrap());
	}

	cv.iter().collect()
}

fn part2(dirs: &Input) -> String {
	let mut cv = Vec::new();
	let mut num = 5;
	for digit in dirs {
		for dir in digit.chars() {
			match num {
				1 => { num = match dir { 'D' => 3, _ => num }},
				2 => { num = match dir { 'R' => 3, 'D' => 6, _ => num }},
				3 => { num = match dir { 'U' => 1, 'R' => 4, 'D' => 7, _ => 2 }},
				4 => { num = match dir { 'L' => 3, 'D' => 8, _ => num }},
				5 => { num = match dir { 'R' => 6, _ => num }},
				6 => { num = match dir { 'U' => 2, 'R' => 7, 'D' => 10, _ => 5 }},
				7 => { num = match dir { 'U' => 3, 'R' => 8, 'D' => 11, _ => 6 }},
				8 => { num = match dir { 'U' => 4, 'R' => 9, 'D' => 12, _ => 7 }},
				9 => { num = match dir { 'L' => 8, _ => num }},
				10 => { num = match dir { 'U' => 6, 'R' => 11, _ => num }},
				11 => { num = match dir { 'U' => 7, 'R' => 12, 'D' => 13, _ => 10 }},
				12 => { num = match dir { 'U' => 8, 'L' => 11, _ => num }},
				_ => { num = match dir { 'U' => 11, _ => num }}
			}
		}
		cv.push(char::from_digit(num, 16).unwrap().to_ascii_uppercase());
	}

	cv.iter().collect()
}

pub fn main() {
	let dirs = parse_input("./input/day2/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&dirs);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&dirs);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day2_test1() {
		let dirs = parse_input("./input/day2/test.txt");
		assert_eq!(part1(&dirs), "1985");
	}

	use super::*;

	#[test]
	fn day2_test2() {
		let dirs = parse_input("./input/day2/test.txt");
		assert_eq!(part2(&dirs), "5DB3");
	}
}
