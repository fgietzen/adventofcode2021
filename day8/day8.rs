use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Segment = HashSet<char>;

fn part1<'a, T: Iterator<Item=&'a Vec<Segment>>>(readings: T) -> usize {
	return readings
		.map(|output_values| {
			return output_values.iter()
				.filter(|v| v.len() == 2 || v.len() == 4 || v.len() == 3 || v.len() == 7)
				.count();
		}).sum();
}

fn determine_segment_value(
	output: &Segment,
	pattern_1: &Segment,
	pattern_4: &Segment,
	pattern_7: &Segment,
) -> String {
	let len_and_1 = output.intersection(pattern_1).count();
	let len_and_4 = output.intersection(pattern_4).count();
	let len_and_7 = output.intersection(pattern_7).count();

	return match (output.len(), len_and_1, len_and_4, len_and_7) {
		(6, 2, 3, 3) => "0",
		(2, _, _, _) => "1",
		(5, 1, 2, 2) => "2",
		(5, 2, 3, 3) => "3",
		(4, _, _, _) => "4",
		(5, 1, 3, 2) => "5",
		(6, 1, 3, 2) => "6",
		(3, _, _, _) => "7",
		(7, _, _, _) => "8",
		(6, 2, 4, 3) => "9",
		(_, _, _, _) => panic!("Invalid segment!")
	}.to_string();
}

fn part2<'a, T: Iterator<Item=&'a (Vec<Segment>, Vec<Segment>)>>(readings: T) -> usize{
	return readings
		.map(|(signal_patterns, output_values)| {
			let pattern_length: HashMap<usize, &Segment> = signal_patterns.iter()
				.map(|p| (p.len(), p))
				.collect();
			let pattern_1 = pattern_length.get(&2).unwrap();
			let pattern_4 = pattern_length.get(&4).unwrap();
			let pattern_7 = pattern_length.get(&3).unwrap();

			return String::from_iter(output_values.iter()
				.map(|value| determine_segment_value(value, pattern_1, pattern_4, pattern_7)));
		})
		.map(|s| s.parse::<usize>().unwrap())
		.sum();
}

fn main() {
	let f = File::open("inputs/day8_input.txt")
		.expect("Could not open file!");

	let readings: Vec<(Vec<Segment>, Vec<Segment>)> = BufReader::new(f).lines()
		.filter_map(|l| l.ok())
		.map(|l| {
			let mut parts = l.split("|")
				.map(|p| p.split_whitespace()
					.map(|v| v.chars().collect())
					.collect::<Vec<Segment>>()
				);

			return (parts.next().unwrap(), parts.next().unwrap());
		})
		.collect();

	println!("Part1: {}", part1(readings.iter().map(|(_, v)| v)));
	println!("Part2: {}", part2(readings.iter()));
}
