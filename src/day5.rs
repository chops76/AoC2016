use std::time::Instant;
use md5::{Md5, Digest};
use std::char;

fn part1(seed: &str) -> String {

	let mut v = Vec::new();
	let mut num:u64 = 0;

	loop {
		let hv = seed.to_string() + &num.to_string();
		let mut hasher = Md5::new();
		hasher.input(hv.as_bytes());
		let result = hasher.result();
		if result[0] == 0 && result[1] == 0 && result[2] < 16 {	
			v.push(char::from_digit(result[2].into(), 16).unwrap());
			if v.len() == 8 {
				break;
			}
		}
		num += 1;
	}

	v.iter().collect()
}

fn part2(seed: &str) -> String {

	let mut v = vec!['-';8];
	let mut num:u64 = 0;

	println!("{}", v.iter().collect::<String>());
	loop {
		let hv = seed.to_string() + &num.to_string();
		let mut hasher = Md5::new();
		hasher.input(hv.as_bytes());
		let result = hasher.result();
		if result[0] == 0 && result[1] == 0 && result[2] < 8 && v[result[2] as usize] == '-' {	
			v[result[2] as usize] = char::from_digit((result[3]/16).into(), 16).unwrap();
			println!("{}", v.iter().collect::<String>());
			if !v.contains(&'-') {
				break;
			}
		}
		num += 1;
	}

	v.iter().collect()
}

pub fn main() {
	
	let p1_timer = Instant::now();
    let p1_result = part1("ffykfhsq");
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2("ffykfhsq");
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

