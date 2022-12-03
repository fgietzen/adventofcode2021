use std::fs::File;
use std::io::{BufRead, BufReader};

mod submarine;

fn part1<'a, T: Iterator<Item=&'a submarine::Command>>(commands: T) -> i32 {
	let (hpos, depth) = commands.fold((0, 0), |(hpos, depth), command|
		match command {
			submarine::Command::HorizontalChange(v) => (hpos + v, depth),
			submarine::Command::DepthChange(v) => (hpos, depth + v)
		});
	return hpos * depth;
}

fn part2<'a, T: Iterator<Item=&'a submarine::Command>>(commands: T) -> i32 {
	let (hpos, _, depth) = commands.fold((0, 0, 0), |(hpos, aim, depth), command|
		match command {
			submarine::Command::HorizontalChange(v) => (hpos + v, aim, depth + v * aim),
			submarine::Command::DepthChange(v) => (hpos, aim + v, depth)
		}
	);
	return hpos * depth;
}

fn main() {
	let f = File::open("inputs/day2_input.txt")
		.expect("Could not open file!");
	let commands: Vec<submarine::Command> = BufReader::new(f).lines()
		.filter_map(|l| l.ok())
		.map(|l| submarine::Command::try_from(l))
		.map(|r| r.expect("Could not parse command!"))
		.collect();

	println!("Part1: {}", part1(commands.iter()));
	println!("Part2: {}", part2(commands.iter()));
}
