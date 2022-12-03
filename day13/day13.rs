use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod camera;

fn part1(manual: &camera::Manual, instruction: &camera::Instruction) -> usize {
	let manual = manual.fold(instruction);

	return manual.number_dots();
}

fn part2<'a, T: Iterator<Item=&'a camera::Instruction>>(
	manual: camera::Manual,
	instructions: T
) -> String {
	return instructions
		.fold(manual, |manual, instruction| manual.fold(instruction))
		.to_string();
}

fn main() {
	let f = File::open("inputs/day13_input.txt").expect("Could not open file!");

	let lines: Vec<String> = BufReader::new(f).lines()
		.filter_map(|l| l.ok())
		.collect();

	let dots: HashSet<camera::Dot> = lines.iter()
		.take_while(|l| !l.is_empty())
		.filter_map(|l| camera::parse_point(l))
		.collect();
	let manual = camera::Manual::new(dots);

	let instructions: Vec<camera::Instruction> = lines.iter()
		.filter(|l| l.contains("fold along") && !l.is_empty())
		.map(|l| l.replace("fold along " ,""))
		.filter_map(|l| camera::Instruction::try_from(l).ok())
		.collect();

	println!("Part1: {}", part1(&manual, &instructions[0]));
	println!("Part2:\n{}", part2(manual, instructions.iter()));
}
