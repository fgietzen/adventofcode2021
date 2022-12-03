use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod image;
use image::InfinityCanvas;

fn part1(enhancement_algorithm: &Vec<bool>, mut canvas: InfinityCanvas) -> usize {
	for _ in 0..2 {
		canvas = canvas.apply_enhancing(&enhancement_algorithm);
	}
	return canvas.len();
}

fn part2(enhancement_algorithm: &Vec<bool>, mut canvas: InfinityCanvas) -> usize {
	for _ in 0..50 {
		canvas = canvas.apply_enhancing(&enhancement_algorithm);
	}
	return canvas.len();
}

fn main() {
	let f = File::open("inputs/day20_input.txt")
		.expect("Could not open file!");
	let mut lines = BufReader::new(f).lines()
		.filter_map(|l| l.ok());

	let enhancement_algorithm: Vec<bool> = lines.next().unwrap().chars()
		.map(|c| match c {
			'#' => true,
			'.' => false,
			_ => panic!("Unexpected character {}", c)
		}).collect();

	let mut points = HashSet::new();
	lines.skip(1)
		.enumerate()
		.for_each(|(i, l)| l.chars()
			.enumerate()
			.filter(|(_, c)| c == &'#')
			.for_each(|(j, _)| { points.insert((i as i32, j as i32)); })
		);
	let canvas = InfinityCanvas::new(points);

	println!("{}", part1(&enhancement_algorithm, canvas.clone()));
	println!("{}", part2(&enhancement_algorithm, canvas));
}
