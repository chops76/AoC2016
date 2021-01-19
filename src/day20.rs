use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;
use std::cmp;

type Input = VecDeque<(u64, u64)>;

fn parse_line(s: &str) -> (u64, u64) {
    let spl = s.split("-").collect::<Vec<&str>>();
    
    (spl[0].parse().unwrap(), spl[1].parse().unwrap())
}

fn parse_input(path: &str) -> Input {
    BufReader::new(File::open(path).unwrap()).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(ranges: &VecDeque<(u64, u64)>) -> u64 {
    let mut r = ranges.clone();
    r.make_contiguous().sort_by_key(|v| v.0);
    while r.len() > 1 {
        if r[0].1+1 >= r[1].0 {
            let v0 = r.pop_front().unwrap();
            let v1 = r.pop_front().unwrap();
            r.push_front((v0.0, cmp::max(v0.1, v1.1)));
        } else {
            break;
        }
    }
    r[0].1+1
}

fn part2(ranges: &VecDeque<(u64, u64)>) -> u64 {
    let mut r = ranges.clone();
    r.make_contiguous().sort_by_key(|v| v.0);
    let mut total = 0;
    while r.len() > 1 {
        if r[0].1+1 >= r[1].0 {
            let v0 = r.pop_front().unwrap();
            let v1 = r.pop_front().unwrap();
            r.push_front((v0.0, cmp::max(v0.1, v1.1)));
        } else {
            let next_val = r[1].0;
            total += next_val - r[0].1 - 1;
            r.pop_front();
        }
    }
    total += 4294967295 - r[0].1;
    total
}

pub fn main() {
    let ranges = parse_input("./input/day20/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&ranges);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&ranges);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

