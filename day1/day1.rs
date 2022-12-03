use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1<'a, T: Iterator<Item=&'a u32>>(lines: T) -> usize {
	return lines
		.fold((0, u32::MAX), |(c, prev), &line| (if line > prev { c + 1 } else { c }, line) )
		.0;
}

fn part2(lines: &Vec<u32>) -> usize {
	return lines.windows(3)
		.map(|w| w[0] + w[1] + w[2])
		.fold((0, u32::MAX), |(c, prev), sum| (if sum > prev { c + 1 } else { c }, sum))
		.0;
}

fn main() {
	let f = File::open("inputs/day1_input.txt")
		.expect("Could not open file!");
	let lines: Vec<u32> = BufReader::new(f).lines()
		.map(|i| i.unwrap().parse::<u32>().unwrap())
		.collect();

	println!("Part1: {}", part1(lines.iter()));
	println!("Part2: {}", part2(&lines));
}
