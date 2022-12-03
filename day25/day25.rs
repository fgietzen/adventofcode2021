use std::fs::File;
use std::io::{BufRead, BufReader};

mod sea_cucumber;
use sea_cucumber::{SeaCucumber, Grid};

fn part1(mut grid: Grid) -> usize {
	let mut step_count = 0;
	loop {
		let (new_grid, moved) = sea_cucumber::step(grid);
		grid = new_grid;
		step_count += 1;

		if !moved {
			break;
		}
	}

	return step_count;
}

fn main() {
	let f = File::open("inputs/day25_input.txt")
		.expect("Could not open file!");
	let values: Vec<Vec<Option<SeaCucumber>>> = BufReader::new(f).lines()
		.filter_map(|l| l.ok())
		.map(|l| l.chars()
			.map(|c| match c {
				'.' => None,
				_ => Some(SeaCucumber::from(c))
			}).collect()
		).collect();
	let grid = Grid::new(values);

	println!("Part1: {}", part1(grid));
}
