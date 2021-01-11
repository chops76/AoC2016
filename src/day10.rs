use std::fs::File;
use std::time::Instant;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cmp;

enum Handoff {
	Bot(u32),
	Output(u32)
}

type Input = (HashMap<u32, Vec<u32>>, HashMap<u32, (Handoff, Handoff)>);

fn parse_line(s: &str, bots: &mut HashMap<u32, Vec<u32>>, inst: &mut HashMap<u32, (Handoff, Handoff)>) {
	let spl: Vec<&str> = s.split(' ').collect();
	if spl[0] == "value" {
		let bot_num = spl[5].parse().unwrap();
		if !bots.contains_key(&bot_num) {
			bots.insert(bot_num, Vec::new());
		} 
		bots.get_mut(&bot_num).unwrap().push(spl[1].parse().unwrap());
	} else {
		let bot_num = spl[1].parse().unwrap();
		let low = match spl[5] {
			"bot" => Handoff::Bot(spl[6].parse().unwrap()),
			_ => Handoff::Output(spl[6].parse().unwrap())
		};
		let high = match spl[10] {
			"bot" => Handoff::Bot(spl[11].parse().unwrap()),
			_ => Handoff::Output(spl[11].parse().unwrap())
		};
		
		inst.insert(bot_num, (low, high));
	}
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	let mut bots = HashMap::new();
	let mut instr = HashMap::new();
	let lines: Vec<String> = BufReader::new(f).lines().flatten().collect();
	for line in lines {
		parse_line(&line, &mut bots, &mut instr);
	}
	(bots, instr)
}

fn part1(bots: &HashMap<u32, Vec<u32>>, instr: &HashMap<u32, (Handoff, Handoff)>) -> u32 {
	let mut bots = bots.clone();
	let mut outputs:HashMap<u32, Vec<u32>> = HashMap::new();
	loop {
		let mut double = 0;
		for (n, b) in &bots {
			if b.len() == 2 {
				double = *n;
				break;
			}
		}

		let a = bots[&double][0];
		let b = bots[&double][1];

		if cmp::min(a, b) == 17 && cmp::max(a, b) == 61 {
			return double;
		}

		bots.insert(double, Vec::new());
		match instr[&double].0 {
			Handoff::Bot(bot_num) => {
				if !bots.contains_key(&bot_num) {
					bots.insert(bot_num, Vec::new());
				} 
				bots.get_mut(&bot_num).unwrap().push(cmp::min(a, b));
			},
			Handoff::Output(output_num) => {
				if !outputs.contains_key(&output_num) {
					outputs.insert(output_num, Vec::new());
				} 
				outputs.get_mut(&output_num).unwrap().push(cmp::min(a, b));
			}
		}

		match instr[&double].1 {
			Handoff::Bot(bot_num) => {
				if !bots.contains_key(&bot_num) {
					bots.insert(bot_num, Vec::new());
				} 
				bots.get_mut(&bot_num).unwrap().push(cmp::max(a, b));
			},
			Handoff::Output(output_num) => {
				if !outputs.contains_key(&output_num) {
					outputs.insert(output_num, Vec::new());
				} 
				outputs.get_mut(&output_num).unwrap().push(cmp::max(a, b));
			}
		}
	}
	0
}

fn part2(bots: &HashMap<u32, Vec<u32>>, instr: &HashMap<u32, (Handoff, Handoff)>) -> u32 {
	let mut bots = bots.clone();
	let mut outputs:HashMap<u32, Vec<u32>> = HashMap::new();
	loop {
		let mut double = 0;
		let mut found = false;
		for (n, b) in &bots {
			if b.len() == 2 {
				double = *n;
				found = true;
				break;
			}
		}

		if !found {
			return outputs[&0][0] * outputs[&1][0] * outputs[&2][0];
		}

		let a = bots[&double][0];
		let b = bots[&double][1];

		bots.insert(double, Vec::new());
		match instr[&double].0 {
			Handoff::Bot(bot_num) => {
				if !bots.contains_key(&bot_num) {
					bots.insert(bot_num, Vec::new());
				} 
				bots.get_mut(&bot_num).unwrap().push(cmp::min(a, b));
			},
			Handoff::Output(output_num) => {
				if !outputs.contains_key(&output_num) {
					outputs.insert(output_num, Vec::new());
				} 
				outputs.get_mut(&output_num).unwrap().push(cmp::min(a, b));
			}
		}

		match instr[&double].1 {
			Handoff::Bot(bot_num) => {
				if !bots.contains_key(&bot_num) {
					bots.insert(bot_num, Vec::new());
				} 
				bots.get_mut(&bot_num).unwrap().push(cmp::max(a, b));
			},
			Handoff::Output(output_num) => {
				if !outputs.contains_key(&output_num) {
					outputs.insert(output_num, Vec::new());
				} 
				outputs.get_mut(&output_num).unwrap().push(cmp::max(a, b));
			}
		}
	}
	0
}

pub fn main() {
	let (bots, instr) = parse_input("./input/day10/input.txt");
	
	let p1_timer = Instant::now();
    let p1_result = part1(&bots, &instr);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&bots, &instr);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day10_test1() {
		let (bots, instr) = parse_input("./input/day10/test.txt");
		assert_eq!(part2(&bots, &instr), 30);
	}

}



