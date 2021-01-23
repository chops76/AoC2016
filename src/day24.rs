use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;
use itertools::Itertools;

type Input = Vec<Vec<char>>;

fn parse_line(s: &str) -> Vec<char> {
    s.chars().collect()    
}

fn parse_input(path: &str) -> Input {
    BufReader::new(File::open(path).unwrap()).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn find_dist(map: &Input, origin: u32) -> Vec<usize>
{
    let mut starting = (0,0);
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x].to_digit(10) == Some(origin) {
                starting = (x, y);
                break;
            }
        }
    }

    let mut distances = vec![0;8];
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((starting, 0));
    seen.insert(starting.clone());
    while queue.len() > 0 {
        let (pos, moves) = queue.pop_front().unwrap();
        if map[pos.1][pos.0] != '.' {
            distances[map[pos.1][pos.0].to_digit(10).unwrap() as usize] = moves;
        }
        if map[(pos.1)-1][pos.0] != '#' && !seen.contains(&(pos.0, (pos.1)-1)) {
            seen.insert((pos.0, (pos.1)-1));
            queue.push_back(((pos.0, (pos.1)-1), moves + 1));
        }
        if map[(pos.1)+1][pos.0] != '#' && !seen.contains(&(pos.0, (pos.1)+1)) {
            seen.insert((pos.0, (pos.1)+1));
            queue.push_back(((pos.0, (pos.1)+1), moves + 1));
        }
        if map[pos.1][(pos.0)-1] != '#' && !seen.contains(&((pos.0)-1, pos.1)) {
            seen.insert(((pos.0)-1, pos.1));
            queue.push_back((((pos.0)-1, pos.1), moves + 1));
        }
        if map[pos.1][(pos.0)+1] != '#' && !seen.contains(&((pos.0)+1, pos.1)) {
            seen.insert(((pos.0)+1, pos.1));
            queue.push_back((((pos.0)+1, pos.1), moves + 1));
        }

    }
    distances
}

fn part1(map: &Input) -> usize {
    let mut distances = Vec::new();
    for digit in 0..8 {
        distances.push(find_dist(map, digit));
    }
    let mut smallest = 10000;
    for order in (1..8).permutations(7) {
        let mut vd = order.iter().map(|v| *v).collect::<VecDeque<usize>>();
        vd.push_front(0);
        let mut total = 0;
        for i in 0..7 {
            total += distances[vd[i]][vd[i+1]];
        }
        if total < smallest {
            smallest = total;
        }
    }
    smallest
}

fn part2(map: &Input) -> usize {
    let mut distances = Vec::new();
    for digit in 0..8 {
        distances.push(find_dist(map, digit));
    }
    let mut smallest = 10000;
    for order in (1..8).permutations(7) {
        let mut vd = order.iter().map(|v| *v).collect::<VecDeque<usize>>();
        vd.push_front(0);
        vd.push_back(0);
        let mut total = 0;
        for i in 0..8 {
            total += distances[vd[i]][vd[i+1]];
        }
        if total < smallest {
            smallest = total;
        }
    }
    smallest
}

pub fn main() {
    let map = parse_input("./input/day24/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&map);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&map);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

