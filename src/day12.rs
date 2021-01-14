use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    cpy,
    inc,
    dec,
    jnz
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
        _ => Instruction::jnz
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
    let path = "./input/day12/input.txt";

    BufReader::new(File::open(path).unwrap()).lines().flatten().map(|s| parse_line(&s)).collect()
}

pub fn main() {
    let prog = parse_input();
    let mut ip: i32 = 0;
	let mut regs:HashMap<String, i32> = HashMap::new();
	regs.insert("a".to_string(), 0);
	regs.insert("b".to_string(), 0);
	regs.insert("c".to_string(), 1);
	regs.insert("d".to_string(), 0);
    while (ip as usize) < prog.len() {
//		println!("{:?}", regs);
//		println!("{}: {:?}", ip, prog[ip as usize]);
       //println!("A: {}  B: {}\n{}: {:?}", regs[0], regs[1], ip, prog[ip as usize]);
        match prog[ip as usize].inst {
            Instruction::cpy => {
				let reg_x = prog[ip as usize].value1.clone();
				let reg_y = prog[ip as usize].value2.clone();
				match prog[ip as usize].value1.parse() {
					Err(_) => regs.insert(reg_y, regs[&reg_x]),
					Ok(num) => regs.insert(reg_y, num)
				};
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
            _ => {
				let reg_x = prog[ip as usize].value1.clone();
				let reg_y = prog[ip as usize].value2.clone();
				let val = match reg_x.parse() {
					Err(_) => regs[&reg_x],
					Ok(num) => num
				};
                if val == 0 {
                    ip += 1;
                } else {
                    ip += reg_y.parse::<i32>().unwrap();
                }
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
