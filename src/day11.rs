use std::time::Instant;
use std::collections::HashSet;
use std::collections::VecDeque;
use itertools::Itertools;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(PartialOrd)]
#[derive(Ord)]
enum Object {
	Chip(char),
	Generator(char)
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Hash)]
struct State {
	elevator: u32,
	floors: Vec<Vec<Object>>
}

fn initial_state() -> State {
	let mut floor1 = Vec::new();
	let mut floor2 = Vec::new();
	let mut floor3 = Vec::new();
	let floor4 = Vec::new();

	floor1.push(Object::Generator('t'));
	floor1.push(Object::Chip('t'));
	floor1.push(Object::Generator('l'));
	floor1.push(Object::Generator('s'));
	floor1.sort();

	floor2.push(Object::Chip('l'));
	floor2.push(Object::Chip('s'));
	floor2.sort();

	floor3.push(Object::Generator('p'));
	floor3.push(Object::Chip('p'));
	floor3.push(Object::Generator('r'));
	floor3.push(Object::Chip('r'));
	floor3.sort();

	State {
		elevator: 0,
		floors: vec![floor1, floor2, floor3, floor4]
	}
}

fn initial_state_part_2() -> State {
	let mut floor1 = Vec::new();
	let mut floor2 = Vec::new();
	let mut floor3 = Vec::new();
	let floor4 = Vec::new();

	floor1.push(Object::Generator('t'));
	floor1.push(Object::Chip('t'));
	floor1.push(Object::Generator('l'));
	floor1.push(Object::Generator('s'));
	floor1.push(Object::Chip('d'));
	floor1.push(Object::Generator('d'));
	floor1.push(Object::Chip('e'));
	floor1.push(Object::Generator('e'));
	floor1.sort();

	floor2.push(Object::Chip('l'));
	floor2.push(Object::Chip('s'));
	floor2.sort();

	floor3.push(Object::Generator('p'));
	floor3.push(Object::Chip('p'));
	floor3.push(Object::Generator('r'));
	floor3.push(Object::Chip('r'));
	floor3.sort();

	State {
		elevator: 0,
		floors: vec![floor1, floor2, floor3, floor4]
	}
}

fn initial_state_test() -> State {
	let mut floor1 = Vec::new();
	let mut floor2 = Vec::new();
	let mut floor3 = Vec::new();
	let floor4 = Vec::new();

	floor1.push(Object::Chip('h'));
	floor1.push(Object::Chip('l'));
	floor1.sort();

	floor2.push(Object::Generator('h'));
	floor2.sort();

	floor3.push(Object::Generator('l'));
	floor3.sort();

	State {
		elevator: 0,
		floors: vec![floor1, floor2, floor3, floor4]
	}
}

fn floor_is_legal(floor: &Vec<Object>) -> bool {
	let mut gens = Vec::new();
	for item in floor {
		match item {
			Object::Generator(name) => {
				gens.push(name);
			}
			_ => {}
		}
	}
	for item in floor {
		match item {
			Object::Chip(name) => {
				if floor.contains(&Object::Generator(*name)) {
					continue;
				} else if gens.len() != 0 {
					return false;
				}
			},
			_ => {}
		}
	}
	true
}

fn legal_state(state: &State) -> bool {
	for floor in &state.floors {
		if !floor_is_legal(&floor) {
			return false;
		}
	}	
	true
}

fn iter(stack: &mut VecDeque<(State, u32)>) -> u32 {
	let mut cur_count = 0;
	let mut hs = HashSet::new();
	while stack.len() > 0 {
		let (state, moves) = stack.pop_front().unwrap();
		if moves != cur_count {
			cur_count = moves;
			println!("Checking {}", moves);
		}
		if state.floors[0].len() == 0 && state.floors[1].len() == 0 && state.floors[2].len() == 0 {
			println!("Found it! - {}", moves);
			return moves;
		}

		if state.floors[state.elevator as usize].len() >= 2 {
			let combos = state.floors[state.elevator as usize].iter().combinations(2);
			for c in combos {
				if state.elevator != 3 {
					let mut new_state = state.clone();
					new_state.elevator += 1;
					let mut tmp = new_state.floors[state.elevator as usize].iter().cloned().collect::<HashSet<Object>>();
					tmp.remove(c[0]);
					tmp.remove(c[1]);
					new_state.floors[state.elevator as usize] = tmp.iter().cloned().collect();
					new_state.floors[state.elevator as usize].sort();
					new_state.floors[new_state.elevator as usize].push(c[0].clone());
					new_state.floors[new_state.elevator as usize].push(c[1].clone());
					new_state.floors[new_state.elevator as usize].sort();
					if legal_state(&new_state) {
						if !hs.contains(&new_state) {
							stack.push_back((new_state.clone(), moves + 1));
							hs.insert(new_state);
						}
					}
				}
				if state.elevator != 0 {
					let mut new_state = state.clone();
					new_state.elevator -= 1;

					let mut tmp = new_state.floors[state.elevator as usize].iter().cloned().collect::<HashSet<Object>>();
					tmp.remove(c[0]);
					tmp.remove(c[1]);
					new_state.floors[state.elevator as usize] = tmp.iter().cloned().collect();
					new_state.floors[state.elevator as usize].sort();
					new_state.floors[new_state.elevator as usize].push(c[0].clone());
					new_state.floors[new_state.elevator as usize].push(c[1].clone());
					new_state.floors[new_state.elevator as usize].sort();
					if legal_state(&new_state) {
						if !hs.contains(&new_state) {
							stack.push_back((new_state.clone(), moves + 1));
							hs.insert(new_state);
						}
					}
				}
			}
		}
		for c in &state.floors[state.elevator as usize] {
			if state.elevator != 3 {
				let mut new_state = state.clone();
				new_state.elevator += 1;
				let mut tmp = new_state.floors[state.elevator as usize].iter().cloned().collect::<HashSet<Object>>();
				tmp.remove(c);
				new_state.floors[state.elevator as usize] = tmp.iter().cloned().collect();
				new_state.floors[state.elevator as usize].sort();
				new_state.floors[new_state.elevator as usize].push(c.clone());
				new_state.floors[new_state.elevator as usize].sort();
				if legal_state(&new_state) {
					if !hs.contains(&new_state) {
						stack.push_back((new_state.clone(), moves + 1));
						hs.insert(new_state);
					}
				}
			}
			if state.elevator != 0 {
				let mut new_state = state.clone();
				new_state.elevator -= 1;

				let mut tmp = new_state.floors[state.elevator as usize].iter().cloned().collect::<HashSet<Object>>();
				tmp.remove(c);
				new_state.floors[state.elevator as usize] = tmp.iter().cloned().collect();
				new_state.floors[state.elevator as usize].sort();
				new_state.floors[new_state.elevator as usize].push(c.clone());
				new_state.floors[new_state.elevator as usize].sort();
				if legal_state(&new_state) {
					if !hs.contains(&new_state) {
						stack.push_back((new_state.clone(), moves + 1));
						hs.insert(new_state);
					}
				}
			}
		}
	}
	0
}

fn part1(state: &State) -> u32 {
	let mut stack = VecDeque::new();
	stack.push_back((state.clone(), 0));
	let moves = iter(&mut stack);
	moves
}

pub fn main() {	
	let p1_timer = Instant::now();
	let state = initial_state();
    let p1_result = part1(&state);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
	let p2_state = initial_state_part_2();
    let p2_result = part1(&p2_state);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
	
}
