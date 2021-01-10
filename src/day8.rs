use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;

#[derive(Debug)]
#[derive(PartialEq)]
enum Instruction {
	RECT,
	ROTATE_COL,
	ROTATE_ROW
}

#[derive(Debug)]
struct Command {
	inst: Instruction,
	val1: usize,
	val2: usize
}

type Input = Vec<Command>;

fn parse_line(s: &str) -> Command {
	let spl: Vec<&str> = s.split(' ').collect();
	if spl[0] == "rect"{
		let coords:Vec<&str> = spl[1].split('x').collect();
		return Command {
			inst: Instruction::RECT,
			val1: coords[0].parse().unwrap(),
			val2: coords[1].parse().unwrap()
		}
	}
	Command {
		inst: if spl[1] == "row" { Instruction::ROTATE_ROW } else { Instruction::ROTATE_COL },
		val1: spl[2][2..].parse().unwrap(),
		val2: spl[4].parse().unwrap()
	}
}

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(commands: &Input) -> u32 {
	let mut grid = vec![vec![false; 50]; 6];
	for cmd in commands {
		if cmd.inst == Instruction::RECT {
			for i in 0..cmd.val1 {
				for j in 0..cmd.val2 {
					grid[j][i] = true;
				}
			}
		} else if cmd.inst == Instruction::ROTATE_ROW {
			let mut vd = VecDeque::new();
			for i in 0..50 {
				vd.push_back(grid[cmd.val1][i]);
			}
			for _ in 0..cmd.val2 {
				let tmp = vd.pop_back().unwrap();
				vd.push_front(tmp);
			}
			for i in 0..50 {
				grid[cmd.val1][i] = vd[i];
			}
		} else if cmd.inst == Instruction::ROTATE_COL {
			let mut vd = VecDeque::new();
			for i in 0..6 {
				vd.push_back(grid[i][cmd.val1]);
			}
			for _ in 0..cmd.val2 {
				let tmp = vd.pop_back().unwrap();
				vd.push_front(tmp);
			}
			for i in 0..6 {
				grid[i][cmd.val1] = vd[i];
			}
		}
	}

	let mut count = 0;
	for j in 0..6 {
		for i in 0..50 {
			if grid[j][i] {
				print!("X");
				count += 1;
			} else {
				print!(".");
			}
		}
		println!("");
	}
	count
}

pub fn main() {
	let commands = parse_input("./input/day8/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&commands);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);
}


