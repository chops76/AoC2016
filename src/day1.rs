use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

type Input = Vec<(i32, i32)>;

fn parse_ins(s: &str) -> (i32, i32) {
	(match s.chars().next().unwrap() { 'R' => 1, _ => -1 }, s[1..].parse().unwrap())
}

fn parse_input() -> Input {
	let path = "./input/day1/input1.txt";
	let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    fstr.split(", ").map(|s| parse_ins(s)).collect()
}

fn part1(dirs: &Input) -> i32 {
	let mut x_pos = 0;
	let mut y_pos = 0;
	let mut facing = 0;
	for dir in dirs {
		facing = (facing + dir.0 + 4) % 4;
		match facing {
			0 => y_pos += dir.1,
			1 => x_pos += dir.1,
			2 => y_pos -= dir.1,
			_ => x_pos -= dir.1
		}
	}

	x_pos.abs() + y_pos.abs()
}

fn part2(dirs: &Input) -> i32 {
	let mut hs = HashSet::new();
	let mut x_pos:i32 = 0;
	let mut y_pos:i32 = 0;
	let mut facing = 0;
	for dir in dirs {
		facing = (facing + dir.0 + 4) % 4;
		for _ in 0..dir.1 {
			match facing {
				0 => y_pos += 1,
				1 => x_pos += 1,
				2 => y_pos -= 1,
				_ => x_pos -= 1
			}
			if hs.contains(&(x_pos, y_pos)) {
				return x_pos.abs() + y_pos.abs();
			}
			hs.insert((x_pos, y_pos));
		}
	}

	x_pos.abs() + y_pos.abs()
}

pub fn main() {
	let dirs = parse_input();

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
	fn day1_test1() {
		let input = "R2, L3".split(", ").map(|s| parse_ins(s)).collect();
		assert_eq!(part1(&input), 5);
	}

	#[test]
	fn day1_test2() {
		let input = "R2, R2, R2".split(", ").map(|s| parse_ins(s)).collect();
		assert_eq!(part1(&input), 2);
	}

	#[test]
	fn day1_test3() {
		let input = "R5, L5, R5, R3".split(", ").map(|s| parse_ins(s)).collect();
		assert_eq!(part1(&input), 12);
	}

	#[test]
	fn day1_test4() {
		let input = "R8, R4, R4, R8".split(", ").map(|s| parse_ins(s)).collect();
		assert_eq!(part2(&input), 4);
	}
}
