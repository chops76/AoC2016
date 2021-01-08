use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::char;

#[derive(Debug)]
struct Room {
	name: String,
	sector: u32, 
	checksum: String
}

type Input = Vec<Room>;

fn parse_line(s: &str) -> Room {
	let spl: Vec<&str> = s.split("[").collect();
	let checksum = spl[1][..spl[1].len()-1].to_string();
	let r = spl[0].to_string().chars().rev().collect::<String>().split("-").next().unwrap().chars().rev().collect::<String>();
	
	Room {
		name: spl[0][..(spl[0].len() - r.len() - 1)].to_string(),
		sector: r.parse().unwrap(),
		checksum: checksum
	}
}

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn checksum(name: &str) -> String {
	let mut v = vec![0;26];
	let n = name.chars().filter(|c| *c != '-').collect::<String>();
	for c in n.as_bytes() {
		v[(c-97) as usize]+=1;
	}
	let mut ret_vec = Vec::new();
	for i in (1..=*v.iter().max().unwrap()).rev() {
		for j in 0..26 {
			if v[j] == i {
				ret_vec.push((j + 97) as u8 as char);
				if ret_vec.len() == 5 {
					return ret_vec.iter().collect();
				}
			}
		}
	}
	"Error".to_string()
}

fn part2(rooms: &Input) -> u32 {
	for room in rooms {
		let mut converted = Vec::new();
		for c in room.name.as_bytes() {
			if *c == 45 {
				converted.push(' ');
			} else {
				converted.push(((((*c as u32 - 97) + room.sector) % 26) + 97) as u8 as char);
			}
		}
		let s = converted.iter().collect::<String>();
		if s == "northpole object storage" {
			return room.sector;
		}
	}
	0
}

fn part1(rooms: &Input) -> u64 {
	rooms.iter().filter(|r| checksum(&r.name) == r.checksum).map(|r| r.sector as u64).sum()
}

pub fn main() {
	let rooms = parse_input("./input/day4/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&rooms);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&rooms);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day4_test1() {
		assert_eq!(checksum("aaaaa-bbb-z-y-x"), "abxyz");
	}

	use super::*;

	#[test]
	fn day4_test2() {
		assert_eq!(checksum("not-a-real-room"), "oarel");
	}
}

