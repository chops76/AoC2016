use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Copy)]
struct Node {
    used: u32,
    total: u32,
    data: bool
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,   
    moves: usize,
    nodes: Vec<Vec<Node>>
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| other.moves.cmp(&self.moves))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Input = Vec<Vec<Node>>;

fn parse_input(path: &str) -> Input {
    let mut empty = Node {
        used: 0,
        total: 0,
        data: false
    };
    let mut nodes = vec![vec![empty; 28];32];
    let lines = BufReader::new(File::open(path).unwrap()).lines().skip(2).flatten().collect::<Vec<String>>();
    for l in lines {
        let spl = l.split_whitespace().collect::<Vec<&str>>();
        let name = spl[0].split("-").collect::<Vec<&str>>();
        let x: usize = name[1][1..].parse().unwrap();
        let y: usize = name[2][1..].parse().unwrap();
        let total = spl[1][..spl[1].len()-1].parse().unwrap();
        let used = spl[2][..spl[2].len()-1].parse().unwrap();
        nodes[x][y].used = used;
        nodes[x][y].total = total;
    }
    nodes[31][0].data = true;
    nodes
}

fn part1(nodes: &Input) -> u64 {
    let mut count = 0;
    for x1 in 0..32 {
        for y1 in 0..28 {
            for x2 in 0..32 {
                for y2 in 0..28 {
                    if x1 == x2 && y1 == y2 {
                        continue;
                    }
                    if nodes[x1][y1].used != 0 && 
                       nodes[x1][y1].used <= nodes[x2][y2].total - nodes[x2][y2].used {
                           count += 1;
                    }
                }
            }
        }
    }
    count
}

fn find_cost(nodes: &Vec<Vec<Node>>) -> usize {
    let mut hole = (0,0);
    let mut goal = (0,0);
    for x in 0..32 {
        for y in 0..28 {
            if nodes[x][y].used == 0 {
                hole = (x, y);
            }
            if nodes[x][y].data {
                goal = (x, y);
            }
        }
    }
    (goal.0 + goal.1) * 100 + (goal.0 as i32 - hole.0 as i32).abs() as usize + 
        (goal.1 as i32 - hole.1 as i32).abs() as usize
}

fn part2(nodes: &Input) -> u64 {
    for y in 0..28 {
        for x in 0..32 {
            if nodes[x][y].used == 0 {
                print!("O ");
            } else if nodes[x][y].used > 100 {
                print!("# ");
            } else if nodes[x][y].data {
                print!("X ");
            } else {
                print!(". ");
            }
        }
        println!("");
    }
    0
}

/*
fn part2(init_nodes: &Input) -> u64 {
    let mut hs = HashSet::new();
    let mut heap = BinaryHeap::new();
    let mut count = 0;
    let mut lowest = 50000;
    heap.push(State { nodes: init_nodes.clone(), cost: 50000, moves: 0 });

//    while count <= 100000 {
    loop {
        count += 1;
//        println!("New one");
        let State { nodes, cost, moves } = heap.pop().unwrap();
        if cost < lowest {
            println!("New one, moves = {}, cost = {}", moves, cost);
            lowest = cost;
        }
        for x in 0..32 {
            for y in 0..28 {
                
                if nodes[x][y].data && x == 0 && y == 0 {
                    println!("Found in {} moves", moves);
                    return moves as u64;
                }
                if nodes[x][y].used == 0 {
                    //println!("Hole at {} {}", x, y);
                    continue;
                }
                if x != 0 && nodes[x][y].used <= nodes[x-1][y].total - nodes[x-1][y].used {
                    let mut new_nodes = nodes.clone();
                    new_nodes[x-1][y].used += new_nodes[x][y].used;
                    new_nodes[x][y].used = 0;
                    new_nodes[x-1][y].data = new_nodes[x][y].data;
                    new_nodes[x][y].data = false;
                    if !hs.contains(&new_nodes) {
                        hs.insert(new_nodes.clone());
                        let cost = find_cost(&new_nodes);
                        heap.push(State { nodes: new_nodes, cost: cost, moves: moves + 1 });
                    }
                    //println!("Moving {} {} left", x, y);
                }
                if y != 0 && nodes[x][y].used <= nodes[x][y-1].total - nodes[x][y-1].used {
                    let mut new_nodes = nodes.clone();
                    new_nodes[x][y-1].used += new_nodes[x][y].used;
                    new_nodes[x][y].used = 0;
                    new_nodes[x][y-1].data = new_nodes[x][y].data;
                    new_nodes[x][y].data = false;
                    if !hs.contains(&new_nodes) {
                        hs.insert(new_nodes.clone());
                        let cost = find_cost(&new_nodes);
                        heap.push(State { nodes: new_nodes, cost: cost, moves: moves + 1 });
                    }
                    //println!("Moving {} {} up", x, y);
                }
                if x != 31 && nodes[x][y].used <= nodes[x+1][y].total - nodes[x+1][y].used {
                    let mut new_nodes = nodes.clone();
                    new_nodes[x+1][y].used += new_nodes[x][y].used;
                    new_nodes[x][y].used = 0;
                    new_nodes[x+1][y].data = new_nodes[x][y].data;
                    new_nodes[x][y].data = false;
                    if !hs.contains(&new_nodes) {
                        hs.insert(new_nodes.clone());
                        let cost = find_cost(&new_nodes);
                        heap.push(State { nodes: new_nodes, cost: cost, moves: moves + 1 });
                    }
                    //println!("Moving {} {} right", x, y);
                }
                if y != 27 && nodes[x][y].used <= nodes[x][y+1].total - nodes[x][y+1].used {
                    let mut new_nodes = nodes.clone();
                    new_nodes[x][y+1].used += new_nodes[x][y].used;
                    new_nodes[x][y].used = 0;
                    new_nodes[x][y+1].data = new_nodes[x][y].data;
                    new_nodes[x][y].data = false;
                    if !hs.contains(&new_nodes) {
                        hs.insert(new_nodes.clone());
                        let cost = find_cost(&new_nodes);
                        heap.push(State { nodes: new_nodes, cost: cost, moves: moves + 1 });
                    }
                    //println!("Moving {} {} down", x, y);
                }
            }
        }
    }
count
}
*/
pub fn main() {
    let nodes = parse_input("./input/day22/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&nodes);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&nodes);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

