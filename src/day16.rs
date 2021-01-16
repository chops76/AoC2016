use std::time::Instant;

fn checksum(s: &str) -> String {
    let ret_str = s.as_bytes().chunks(2).map(|c| if c[0] == c[1] {'1'} else {'0'}).collect::<String>();
    
    if ret_str.len() % 2 == 0 {
        checksum(&ret_str)
    } else {
        ret_str
    }
}

fn part1(seed: &str) -> String {
    let mut ret_str = seed.to_string();
    while ret_str.len() < 272 {
        let flipped = ret_str.chars().rev().map(|c| match c { '0' => '1', _ => '0'}).collect::<String>();
        ret_str += "0";
        ret_str += &flipped;
    }
    ret_str = ret_str[..272].to_string();
    checksum(&ret_str)
}

fn part2(seed: &str) -> String {
    let mut ret_str = seed.to_string();
    while ret_str.len() < 35651584 {
        let flipped = ret_str.chars().rev().map(|c| match c { '0' => '1', _ => '0'}).collect::<String>();
        ret_str += "0";
        ret_str += &flipped;
    }
    ret_str = ret_str[..35651584].to_string();
    checksum(&ret_str)
}

pub fn main() {
    let p1_timer = Instant::now();
    let p1_result = part1("10001110011110000");
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2("10001110011110000");
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn day16_test1() {
        assert_eq!(checksum("110010110100"),"100");
    }

    #[test]
	fn day16_test2() {
        assert_eq!(checksum("10000011110010000111"),"01100");
    }
}
