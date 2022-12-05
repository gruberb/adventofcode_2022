use std::fs;
use regex::Regex;



fn main() {
	let contents = fs::read_to_string("input.txt")
		.expect("Should have been able to read the file");

	let (stack, instructions) = contents.split_once("\n\n").unwrap();

	let re = Regex::new(r"\d+").unwrap();

	let first  = stack.lines().rev().next().unwrap();

	let stacks: Vec<char> = re.find_iter(first).map(|m| m.as_str().parse::<char>().unwrap()).collect();

	let stack_contents = stack.lines().rev().skip(1);

	let mut master_stack: Vec<Vec<char>> = vec![];

	for _ in stacks {
		master_stack.push(Vec::default());
	}

	for l in stack_contents {
		for (n, c) in l.chars().skip(1).step_by(4).enumerate() {
			if ('A'..='Z').contains(&c) {
				master_stack[n].push(c);
			}
		}
	}

	// // PART 1 SOLUTION
	// for l in instructions.lines() {
	// 	let re = Regex::new(r"\d+").unwrap();
	// 	let in_numbers: Vec<usize> = re.find_iter(l).map(|m| m.as_str().parse::<usize>().unwrap()).collect();

	// 	part_1( &mut master_stack, in_numbers[0] as i32, in_numbers[1] - 1, in_numbers[2] - 1);
	// }

	// for s in master_stack.iter_mut() {
	// 	print!("{}", s.pop().unwrap());
	// }

	// PART 2 SOLUTION
	for l in instructions.lines() {
		let re = Regex::new(r"\d+").unwrap();
		let in_numbers: Vec<usize> = re.find_iter(l).map(|m| m.as_str().parse::<usize>().unwrap()).collect();

		part_2( &mut master_stack, in_numbers[0] as i32, in_numbers[1] - 1, in_numbers[2] - 1);
	}

	for s in master_stack.iter_mut() {
		print!("{}", s.pop().unwrap());
	}
	


}

// fn part_1(stacks: &mut Vec<Vec<char>>, amount: i32, from: usize, to: usize) {
// 	for _ in 1..=amount {
// 		let element = stacks[from].pop().unwrap();
// 		stacks[to].push(element);
// 	}
// }

fn part_2(stacks: &mut Vec<Vec<char>>, amount: i32, from: usize, to: usize) {
	println!("Move {} from {} to {}", amount, from, to);
	if amount == 1 {
		let element = stacks[from].pop().unwrap();
		stacks[to].push(element);
	}

	if amount > 1 {
		let mut crates: Vec<char> = Vec::new();
		for _ in 1..=amount {
			crates.push(stacks[from].pop().unwrap());
		}

		crates.reverse();

		for c in crates.iter() {
			stacks[to].push(*c);
		}
		
	}
}