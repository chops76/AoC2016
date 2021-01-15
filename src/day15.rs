use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<(u32, u32, u32)>;

fn parse_line(s: &str) -> (u32, u32, u32) {
    let spl = s.split(" ").collect::<Vec<&str>>();
    
    (spl[1][1..].parse().unwrap(), spl[3].parse().unwrap(), spl[11][..spl[11].len()-1].parse().unwrap())
}

fn parse_input(path: &str) -> Input {
    BufReader::new(File::open(path).unwrap()).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(capsules: &Input) -> u32 {
    let mut count = 0;
    loop {
        let mut found = true;
        for c in capsules {
            if (c.2 + c.0 + count) % c.1 != 0 {
                found = false;
                break;
            }
        }
        if found {
            return count;
        }
        count += 1;
    }
}

fn part2(capsules: &Input) -> u32 {
    let mut new_cap = capsules.clone();
    new_cap.push((capsules.len() as u32 + 1, 11, 0));
    let mut count = 0;
    loop {
        let mut found = true;
        for c in &new_cap {
            if (c.2 + c.0 + count) % c.1 != 0 {
                found = false;
                break;
            }
        }
        if found {
            return count;
        }
        count += 1;
    }
}

pub fn main() {
    let capsules = parse_input("./input/day15/input.txt");
    println!("{:?}", capsules);

    let p1_timer = Instant::now();
    let p1_result = part1(&capsules);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&capsules);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn day15_test1() {
        let capsules = parse_input("./input/day15/test.txt");
        assert_eq!(part1(&capsules),5);
    }

    #[test]
	fn day15_test2() {
        let capsules = parse_input("./input/day15/test.txt");
        assert_eq!(part2(&capsules),85);
    }
}
