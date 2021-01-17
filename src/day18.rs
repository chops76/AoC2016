use std::io::Read;
use std::fs::File;
use std::time::Instant;

fn parse_input(path: &str) -> Vec<char> {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
	fstr.chars().collect()
}

fn get_next(cur_line: &Vec<char>) -> Vec<char> {
	let mut ret_vec = Vec::new();
	for i in 0..cur_line.len() {
		let l = i != 0 && cur_line[i-1] == '^';
		let c = cur_line[i] == '^';
		let r = i != cur_line.len() - 1 && cur_line[i+1] == '^';
		if (l && c && !r) || (!l && c && r) || (l && !c && !r) || (!l && !c && r)
		{
			ret_vec.push('^');
		} else {
			ret_vec.push('.');
		}
	}

	ret_vec
}

fn part1(first_line: &Vec<char>) -> usize {
	let mut line = first_line.clone();
	let mut count = 0;
	for _ in 0..40 {
		count += line.iter().filter(|c| **c == '.').count();
		line = get_next(&line);
	}
	count
}

fn part2(first_line: &Vec<char>) -> usize {
	let mut line = first_line.clone();
	let mut count = 0;
	for _ in 0..400000 {
		count += line.iter().filter(|c| **c == '.').count();
		line = get_next(&line);
	}
	count
}

pub fn main() {
	let first_line = parse_input("./input/day18/input.txt");
	
	let p1_timer = Instant::now();
    let p1_result = part1(&first_line);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&first_line);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day18_test1() {
		assert_eq!(get_next(&".^^^^".chars().collect()), "^^..^".chars().collect::<Vec<char>>());
	}

	#[test]
	fn day18_test2() {
		assert_eq!(get_next(&"..^^.".chars().collect()), ".^^^^".chars().collect::<Vec<char>>());
	}
}



