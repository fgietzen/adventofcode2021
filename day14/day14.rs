use std::fs::File;
use std::io::{BufRead, BufReader};

mod polymerization;
use crate::polymerization::{Counter, Instruction, Polymer};

fn perform_pair_insertion(
	mut element_count: Counter<char>,
	mut polymers: Counter<Polymer>,
	instructions: &Vec<Instruction>,
	iterations: usize
) -> i64 {
	for _ in 0..iterations {
		let mut new_polymers = polymers.clone();
		for instruction in instructions {
			let current_amount = polymers.count(instruction.rule());

			element_count.add(instruction.result(), current_amount);

			new_polymers.minus(instruction.rule().clone(), current_amount);
			instruction.rule().morph(instruction.result()).iter()
				.for_each(|r| new_polymers.add(r.clone(), current_amount));
		}
		polymers = new_polymers;
	}

	let max = element_count.max().map(|(_, v)| v).unwrap_or(0);
	let min = element_count.min().map(|(_, v)| v).unwrap_or(0);
	return max - min;
}

fn part1(
	element_count: Counter<char>,
	polymers: Counter<Polymer>,
	instructions: &Vec<Instruction>
) -> i64 {
	return perform_pair_insertion(element_count, polymers, instructions, 10);
}

fn part2(
	element_count: Counter<char>,
	polymers: Counter<Polymer>,
	instructions: &Vec<Instruction>
) -> i64 {
	return perform_pair_insertion(element_count, polymers, instructions, 40);
}

fn main() {
	let f = File::open("inputs/day14_input.txt").expect("Could not open file!");

	let mut lines = BufReader::new(f).lines()
		.filter_map(|l| l.ok());

	let mut element_count = Counter::new();
	let mut polymers = Counter::new();
	let elements = lines.next().unwrap()
		.chars()
		.collect::<Vec<char>>();
	elements.iter()
		.for_each(|&element| element_count.add(element, 1));
	elements.windows(2)
		.for_each(|polymer| polymers.add(Polymer::new(polymer[0], polymer[1]), 1));

	let lines = lines.skip_while(|l| l.is_empty());
	let instructions: Vec<Instruction> = lines
		.filter_map(|l| Instruction::try_from(l.as_str()).ok())
		.collect();

	println!("Part1: {}", part1(element_count.clone(), polymers.clone(), &instructions));
	println!("Part1: {}", part2(element_count, polymers, &instructions));
}
