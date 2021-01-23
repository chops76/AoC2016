use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
enum Instruction {
    cpy,
    inc,
    dec,
    jnz,
    tgl
}

#[derive(Debug)]
struct Command {
    inst: Instruction,
    value1: String,
    value2: String
}

fn parse_line(s: &str) -> Command {
//	println!("Parse Line {}", s);
    let spl = s.split(" ").collect::<Vec<&str>>();
    
    let cmd = match spl[0] {
        "cpy" => Instruction::cpy,
        "inc" => Instruction::inc,
        "dec" => Instruction::dec,
        "jnz" => Instruction::jnz,
        _ => Instruction::tgl
    };
	let val1 = spl[1].to_string(); 
    let val2 = if spl.len() > 2 { spl[2].to_string() } else { String::new() };

    Command {
        inst: cmd,
        value1: val1,
        value2: val2
    }
}

fn parse_input() -> Vec<Command> {
    let path = "./input/day23/input.txt";

    BufReader::new(File::open(path).unwrap()).lines().flatten().map(|s| parse_line(&s)).collect()
}

pub fn main() {
    let mut prog = parse_input();
    let mut ip: i32 = 0;
	let mut regs:HashMap<String, i32> = HashMap::new();
	regs.insert("a".to_string(), 12);
	regs.insert("b".to_string(), 0);
	regs.insert("c".to_string(), 0);
	regs.insert("d".to_string(), 0);
    while (ip as usize) < prog.len() {
//		println!("{:?}", regs);
		//println!("{}: {:?}", ip, prog[ip as usize]);
        //println!("A: {}  B: {}\n{}: {:?}", regs[0], regs[1], ip, prog[ip as usize]);
        match prog[ip as usize].inst {
            Instruction::cpy => {
				let reg_x = prog[ip as usize].value1.clone();
                let reg_y = prog[ip as usize].value2.clone();
                if reg_y == "a" || reg_y == "b" || reg_y == "c" || reg_y == "d" {
                    match prog[ip as usize].value1.parse() {
                        Err(_) => regs.insert(reg_y, regs[&reg_x]),
                        Ok(num) => regs.insert(reg_y, num)
                    };
                }
                ip += 1;
            },
            Instruction::inc => {
				let reg_x = prog[ip as usize].value1.clone();
				let cur_val = regs[&reg_x];
				regs.insert(reg_x, cur_val + 1);
                ip += 1;
            },
            Instruction::dec => {
				let reg_x = prog[ip as usize].value1.clone();
				let cur_val = regs[&reg_x];
				regs.insert(reg_x, cur_val - 1);
                ip += 1;
            },
            Instruction::jnz => {
				let reg_x = prog[ip as usize].value1.clone();
				let reg_y = prog[ip as usize].value2.clone();
				let val = match reg_x.parse() {
					Err(_) => regs[&reg_x],
					Ok(num) => num
                };
                let offset:i32 = match reg_y.parse() {
					Err(_) => regs[&reg_y],
					Ok(num) => num
                };
                if val == 0 {
                    ip += 1;
                } else {
                    ip += offset;
                }
            },
            _ => {
                let reg_x = regs[&prog[ip as usize].value1];
                let to_toggle = ip + reg_x;
                if (to_toggle as usize) < prog.len() {
                    let inst = prog[to_toggle as usize].inst.clone();
                    match inst {
                        Instruction::dec => { prog[to_toggle as usize].inst = Instruction::inc },
                        Instruction::inc => { prog[to_toggle as usize].inst = Instruction::dec },
                        Instruction::tgl => { prog[to_toggle as usize].inst = Instruction::inc },
                        Instruction::jnz => { prog[to_toggle as usize].inst = Instruction::cpy },
                        Instruction::cpy => { prog[to_toggle as usize].inst = Instruction::jnz }
                    }
                }
                ip += 1;
            }
        }
    }
    println!("Part 2: {}", regs["a"]);
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn day12_test1() {
        assert_eq!(0,0);
    }
}
