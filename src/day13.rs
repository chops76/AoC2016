use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;

fn empty_spot(x: u32, y: u32) -> bool {
    let mut val: u32 = x * x + 3 * x + 2 * x * y + y + y * y + 1350;
    let mut count = 0;
    while val != 0 {
        if val % 2 == 1 {
            count += 1;
        }
        val /= 2;
    }
    count % 2 == 0
}

fn part1() -> u32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((1, 1, 0));
    while queue.len() != 0 {
        let (x, y, moves) = queue.pop_front().unwrap();
        if x == 31 && y == 39 {
            return moves;
        }
        visited.insert((x, y));

        if !visited.contains(&(x+1,y)) && empty_spot(x+1, y) {
            queue.push_back((x+1,y,moves+1));
        }
        if !visited.contains(&(x,y+1)) && empty_spot(x, y+1) {
            queue.push_back((x,y+1,moves+1));
        }
        if x != 0 && !visited.contains(&(x-1,y)) && empty_spot(x-1, y) {
            queue.push_back((x-1,y,moves+1));
        }
        if y != 0 && !visited.contains(&(x,y-1)) && empty_spot(x, y-1) {
            queue.push_back((x,y-1,moves+1));
        }
    }
    0
}

fn part2() -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((1, 1, 0));
    loop {
        let (x, y, moves) = queue.pop_front().unwrap();
        if moves == 51 {
            return visited.len();
        }
        visited.insert((x, y));

        if !visited.contains(&(x+1,y)) && empty_spot(x+1, y) {
            queue.push_back((x+1,y,moves+1));
        }
        if !visited.contains(&(x,y+1)) && empty_spot(x, y+1) {
            queue.push_back((x,y+1,moves+1));
        }
        if x != 0 && !visited.contains(&(x-1,y)) && empty_spot(x-1, y) {
            queue.push_back((x-1,y,moves+1));
        }
        if y != 0 && !visited.contains(&(x,y-1)) && empty_spot(x, y-1) {
            queue.push_back((x,y-1,moves+1));
        }
    }
}

pub fn main() {
    let p1_timer = Instant::now();
    let p1_result = part1();
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2();
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn day12_test1() {
        assert_eq!(0,0);
    }
}
