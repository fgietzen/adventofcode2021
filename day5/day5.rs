use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod lines;

fn part1<'a, T: Iterator<Item=&'a (lines::Point, lines::Point)>>(lines: T) -> usize {
	let mut grid = HashMap::new();

	let points = lines
		.filter(|(start, end)| (start.0 == end.0) || (start.1 == end.1))
		.flat_map(|l| lines::points_on_line(l.0, l.1));

	for p in points {
		*grid.entry(p).or_insert(0) += 1;
	}
	return grid.values()
		.filter(|&&i| i >= 2)
		.count();
}

fn part2<'a, T: Iterator<Item=&'a (lines::Point, lines::Point)>>(lines: T) -> usize {
	let mut grid = HashMap::new();

	let points = lines
		.flat_map(|l| lines::points_on_line(l.0, l.1));

	for p in points {
		*grid.entry(p).or_insert(0) += 1;
	}
	return grid.values()
		.filter(|&&i| i >= 2)
		.count();
}

fn main() {
	let f = File::open("inputs/day5_input.txt").expect("Could not open file!");

	let points: Vec<(lines::Point, lines::Point)> = BufReader::new(f).lines()
		.filter_map(|l| lines::parse_line(&l.unwrap()))
		.collect();

	println!("Part1: {}", part1(points.iter()));
	println!("Part2: {}", part2(points.iter()));
}
