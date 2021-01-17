use std::time::Instant;

fn part1(elves: usize) -> usize {
	let mut elf_list = Vec::with_capacity(elves);
	for i in 0..elves-1 {
		elf_list.push(i+1);
	}
	elf_list.push(0);
	let mut count = 0;
	let mut cur = 0;
	loop {
		elf_list[cur] = elf_list[elf_list[cur]];
		count += 1;
		if count == elves - 1 {
			return cur + 1;
		}
		cur = elf_list[cur];
	}
}

fn part2(elves: usize) -> usize {
	let mut elf_list = Vec::with_capacity(elves);
	for i in 0..elves-1 {
		elf_list.push(i+1);
	}
	elf_list.push(0);
	let mut count = 0;
	let mut cur = 0;
	loop {
		if count % 1000 == 0 {
			println!("Removed {}", count);
		}
		let to_iter = ((elves - count) / 2) - 1;
		let mut it = cur;
		for i in 0..to_iter {
			it = elf_list[it];
		}
		elf_list[it] = elf_list[elf_list[it]];
		count += 1;
		if count == elves - 1 {
			return cur + 1;
		}
		cur = elf_list[cur];
	}
}

pub fn main() {
	let p1_timer = Instant::now();
	let p1_result = part1(3014603);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
	let p2_result = part2(3014603);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}




