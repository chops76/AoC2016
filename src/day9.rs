use std::io::Read;
use std::fs::File;
use std::time::Instant;

fn parse_input(path: &str) -> String {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
	fstr
}

fn part1(compressed: &str) -> usize {
	let mut uncompressed = Vec::new();
	let compressed_chars = compressed.chars().collect::<Vec<char>>();
	let mut cur_pos = 0;
	while cur_pos < compressed_chars.len() {
		if compressed_chars[cur_pos] != '(' {
			uncompressed.push(compressed_chars[cur_pos]);
			cur_pos += 1;
			continue;
		}

		cur_pos += 1;
		let mut tmp = Vec::new();
		while compressed_chars[cur_pos] != 'x' {
			tmp.push(compressed_chars[cur_pos]);
			cur_pos += 1;
		}
		let num_chars: usize = tmp.iter().collect::<String>().parse().unwrap();

		cur_pos += 1;
		tmp = Vec::new();
		while compressed_chars[cur_pos] != ')' {
			tmp.push(compressed_chars[cur_pos]);
			cur_pos += 1;
		}
		let num_times: usize = tmp.iter().collect::<String>().parse().unwrap();

		cur_pos += 1;
		tmp = Vec::new();
		for i in 0..num_chars {
			tmp.push(compressed_chars[cur_pos]);
			cur_pos += 1;
		}
		for i in 0..num_times {
			uncompressed.append(&mut tmp.clone());
		}
	}

	uncompressed.len()
}

fn part2(compressed: &str) -> u64
{
	let mut uncompressed_len = 0;
	let compressed_chars = compressed.chars().collect::<Vec<char>>();
	let mut cur_pos = 0;
	while cur_pos < compressed_chars.len() {
		if compressed_chars[cur_pos] != '(' {
			uncompressed_len += 1;
			cur_pos += 1;
			continue;
		}

		cur_pos += 1;
		let mut tmp = Vec::new();
		while compressed_chars[cur_pos] != 'x' {
			tmp.push(compressed_chars[cur_pos]);
			cur_pos += 1;
		}
		let num_chars: usize = tmp.iter().collect::<String>().parse().unwrap();

		cur_pos += 1;
		tmp = Vec::new();
		while compressed_chars[cur_pos] != ')' {
			tmp.push(compressed_chars[cur_pos]);
			cur_pos += 1;
		}
		let num_times: usize = tmp.iter().collect::<String>().parse().unwrap();

		cur_pos += 1;
		tmp = Vec::new();
		for i in 0..num_chars {
			tmp.push(compressed_chars[cur_pos]);
			cur_pos += 1;
		}
		uncompressed_len += num_times as u64 * part2(&tmp.iter().collect::<String>());
	}
	
	uncompressed_len
}

pub fn main() {
	let compressed = parse_input("./input/day9/input.txt");
	
	let p1_timer = Instant::now();
    let p1_result = part1(&compressed);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&compressed);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day9_test1() {
		assert_eq!(part1("ADVENT"), 6);
	}

	#[test]
	fn day9_test2() {
		assert_eq!(part1("A(1x5)BC"), 7);
	}

	#[test]
	fn day9_test3() {
		assert_eq!(part1("(3x3)XYZ"), 9);
	}

	#[test]
	fn day9_test4() {
		assert_eq!(part1("A(2x2)BCD(2x2)EFG"), 11);
	}

	#[test]
	fn day9_test5() {
		assert_eq!(part1("(6x1)(1x3)A"), 6);
	}

	#[test]
	fn day9_test6() {
		assert_eq!(part1("X(8x2)(3x3)ABCY"), 18);
	}

	#[test]
	fn day9_test7() {
		assert_eq!(part2("(3x3)XYZ"), 9);
	}

	#[test]
	fn day9_test8() {
		assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
	}

	#[test]
	fn day9_test9() {
		assert_eq!(part2("X(8x2)(3x3)ABCY"), 20);
	}

	#[test]
	fn day9_test10() {
		assert_eq!(part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
	}
}



