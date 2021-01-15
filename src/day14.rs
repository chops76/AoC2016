use std::time::Instant;
use md5::{Md5, Digest};
use std::collections::VecDeque;

fn contains_triple(s: &str) -> (bool, u8) {
	for i in 0..s.len()-2 {
		if s.as_bytes()[i] == s.as_bytes()[i+1] && s.as_bytes()[i] == s.as_bytes()[i+2] {
			return (true, s.as_bytes()[i]);
		}
	}
	(false, 0)
}

fn contains_five(s: &str, c: u8) -> bool {
	for i in 0..s.len()-4 {
		if s.as_bytes()[i] == c && s.as_bytes()[i+1] == c && s.as_bytes()[i+2] == c &&
		   s.as_bytes()[i+3] == c && s.as_bytes()[i+4] == c  {
			return true;
		}
	}
	false
}

fn super_hash(s: &str) -> String {
	let mut hash_str = s.to_string();

	for _ in 0..2017 {
		let mut hasher = Md5::new();
		hasher.input(hash_str.as_bytes());
		hash_str = format!("{:032x}", hasher.result());
	}

	hash_str
}

fn part1(seed: &str) -> usize {
	let mut vd:VecDeque<String> = VecDeque::new();
	let mut num = 0;
	let mut num_found = 0;

	loop {
		let hash_str: String;
		if num < vd.len() {
			hash_str = vd[num].clone();
		} else {
			let hv = seed.to_string() + &num.to_string();
			let mut hasher = Md5::new();
			hasher.input(hv.as_bytes());
			hash_str = format!("{:032x}", hasher.result());
			vd.push_back(hash_str.clone());
		}

		let (found, rpt_char) = contains_triple(&hash_str);
		if found {
			for i in 1..=1000 {
				let test_str;
				if i + num >= vd.len() {
					let mut hasher = Md5::new();
					let hv = seed.to_string() + &(num + i).to_string();
					hasher.input(hv.as_bytes());
					test_str = format!("{:032x}", hasher.result());
					vd.push_back(test_str.clone());					
				} else {
					test_str = vd[num+i].clone();
				}
				if contains_five(&test_str, rpt_char) {
					num_found += 1;
					if num_found == 64 {
						return num;
					}
					break;
				}
			}
		}
		num += 1;
	}
}

fn part2(seed: &str) -> usize {
	let mut vd:VecDeque<String> = VecDeque::new();
	let mut num = 0;
	let mut num_found = 0;

	loop {
		let hash_str: String;
		if num < vd.len() {
			hash_str = vd[num].clone();
		} else {
			let hv = seed.to_string() + &num.to_string();
			hash_str = super_hash(&hv);
			vd.push_back(hash_str.clone());
		}

		let (found, rpt_char) = contains_triple(&hash_str);
		if found {
			for i in 1..=1000 {
				let test_str;
				if i + num >= vd.len() {
					let hv = seed.to_string() + &(num + i).to_string();
					test_str = super_hash(&hv);
					vd.push_back(test_str.clone());					
				} else {
					test_str = vd[num+i].clone();
				}
				if contains_five(&test_str, rpt_char) {
					num_found += 1;
					if num_found == 64 {
						return num;
					}
					break;
				}
			}
		}
		num += 1;
	}
}

pub fn main() {
	let p1_timer = Instant::now();
    let p1_result = part1("ahsbgdzn");
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
	let p2_result = part2("ahsbgdzn");
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

