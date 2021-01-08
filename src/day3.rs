use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::char;

type Input = Vec<Vec<u32>>;

fn parse_line(s: &str) -> Vec<u32> {
	s.split_whitespace().map(|v| v.parse()).flatten().collect()
}

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn valid(sides: &Vec<u32>) -> bool {
	let mut s = sides.clone();
	s.sort();
	s[0] + s[1] > s[2]
}

fn part1(triangles: &Input) -> usize {
	triangles.iter().filter(|v| valid(v)).count()
}

fn transform_vec(triangles: &Input) -> Vec<Vec<u32>> {
	let mut ret_vec = Vec::new();
	let mut count = 0;
	while count < triangles.len() {
		ret_vec.push(vec![triangles[count][0], triangles[count+1][0], triangles[count+2][0]]);
		ret_vec.push(vec![triangles[count][1], triangles[count+1][1], triangles[count+2][1]]);
		ret_vec.push(vec![triangles[count][2], triangles[count+1][2], triangles[count+2][2]]);
		count += 3;
	}

	ret_vec
}

fn part2(triangles: &Input) -> usize {
	let new_tri = transform_vec(triangles);
	new_tri.iter().filter(|v| valid(v)).count()
}

pub fn main() {
	let triangles = parse_input("./input/day3/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&triangles);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&triangles);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day3_test1() {
		assert_eq!(valid(&vec![5, 10, 25]), false);
	}
}
