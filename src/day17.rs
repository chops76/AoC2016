use std::time::Instant;
use md5::{Md5, Digest};
use std::collections::VecDeque;

fn exits(s: &str) -> (bool,bool,bool,bool) {
    let mut hasher = Md5::new();
    hasher.input(s.as_bytes());
    let result = hasher.result();
    ((result[0] / 16) > 10, (result[0] % 16) > 10, 
     (result[1] / 16) > 10, (result[1] % 16) > 10)
}

fn part1(seed: &str) -> String {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, "".to_string()));
    while queue.len() != 0 {
        let (x, y, path) = queue.pop_front().unwrap();
        let exit_list = exits(&(seed.to_string() + &path));
        if y != 0 && exit_list.0 {
            queue.push_back((x, y-1, path.clone() + "U"));
        }
        if y != 3 && exit_list.1 {
            if x == 3 && y == 2 {
                return path + "D";
            }
            queue.push_back((x, y+1, path.clone() + "D"));
        }
        if x != 0 && exit_list.2 {
            queue.push_back((x-1, y, path.clone() + "L"));
        }
        if x != 3 && exit_list.3 {
            if x == 2 && y == 3 {
                return path + "R";
            }
            queue.push_back((x+1, y, path.clone() + "R"));
        }
    }
    String::new()
}

fn part2(seed: &str) -> usize {
    let mut queue = VecDeque::new();
    let mut max_len = 0;
    queue.push_back((0, 0, "".to_string()));
    while queue.len() != 0 {
        let (x, y, path) = queue.pop_front().unwrap();
        let exit_list = exits(&(seed.to_string() + &path));
        if y != 0 && exit_list.0 {
            queue.push_back((x, y-1, path.clone() + "U"));
        }
        if y != 3 && exit_list.1 {
            if x == 3 && y == 2 {
                if max_len < path.len() + 1 {
                    max_len = path.len() + 1;
                }
            } else {
                queue.push_back((x, y+1, path.clone() + "D"));
            }
        }
        if x != 0 && exit_list.2 {
            queue.push_back((x-1, y, path.clone() + "L"));
        }
        if x != 3 && exit_list.3 {
            if x == 2 && y == 3 {
                if max_len < path.len() + 1 {
                    max_len = path.len() + 1;
                }
            } else {
                queue.push_back((x+1, y, path.clone() + "R"));
            }
        }
    }
    max_len
}

pub fn main() {
    let p1_timer = Instant::now();
    let p1_result = part1("pslxynzg");
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2("pslxynzg");
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn day17_test1() {
        assert_eq!(part1("ihgpwlah"),"DDRRRD");
    }

	#[test]
	fn day17_test2() {
        assert_eq!(part1("kglvqrro"),"DDUDRLRRUDRD");
    }

    #[test]
	fn day17_test3() {
        assert_eq!(part1("ulqzkmiv"),"DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
	fn day17_test4() {
        assert_eq!(part2("ihgpwlah"),370);
    }

	#[test]
	fn day17_test5() {
        assert_eq!(part2("kglvqrro"),492);
    }

    #[test]
	fn day17_test6() {
        assert_eq!(part2("ulqzkmiv"),830);
    }
}
