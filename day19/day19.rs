use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

mod scanners;
use scanners::Point;

fn parse_coordinate(p: &str) -> Point {
	let mut split = p.split(",")
		.map(|v| v.parse::<i32>().unwrap());

	return (split.next().unwrap(), split.next().unwrap(), split.next().unwrap());
}

fn part1(beacons: &HashSet<Point>) -> usize {
	return beacons.len();
}

fn part2(scanners: &Vec<Point>) -> u32 {
	return (0..scanners.len())
		.map(|i| (0..scanners.len())
			.map(move |j| scanners::distance(&scanners[i], &scanners[j]))
	).flatten()
	.max().unwrap_or(0);
}

fn main() {
	let f = File::open("inputs/day19_input.txt").expect("Could not open file!");
	let lines = BufReader::new(f).lines()
		.filter_map(|l| l.ok());

	let mut scanners = Vec::new();
	let mut beacons = Vec::new();
	for line in lines {
		if line == "" {
			scanners.push(beacons);
			beacons = Vec::new();
			continue;
		}
		if line.starts_with("---") {
			continue;
		}
		beacons.push(parse_coordinate(&line));
	}
	scanners.push(beacons);

	let (known_scanners, known_beacons) = scanners::reconstruct_scanner_map(scanners);

	println!("Part1: {}", part1(&known_beacons));
	println!("Part2: {}", part2(&known_scanners));
}
