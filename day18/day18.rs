use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

mod snailfish_math;
use snailfish_math::Number;

fn part1<T: Iterator<Item=Number>>(mut values: T) -> u64 {
	let mut value = values.next().unwrap();
	for n in values {
		value = value + n;
	}
	return value.magnitude();
}

fn part2(values: Vec<Number>) -> u64 {
	let indices: Vec<usize> = (0..values.len()).collect();
	let indices: Vec<Vec<usize>> = indices.chunks(12)
		.map(|chunk| chunk.into())
		.collect();

	let values = Arc::new(values);

	let mut handles = Vec::new();
	for range in indices {
		let val = values.clone();

		handles.push(std::thread::spawn(move || {
			return range.iter()
				.flat_map(|i| (0..val.len())
					.map(|j| (*i, j))
				)
				.filter(|(i, j)| i != j)
				.map(|(i, j)| val[i].clone() + val[j].clone())
				.map(|n| n.magnitude())
				.max().unwrap_or(0);
		}));
	}

	return handles.into_iter()
		.map(|h| h.join().unwrap())
		.max().unwrap();
}

fn main() {
	let f = File::open("inputs/day18_input.txt")
		.expect("Could not open file!");
	let values: Vec<Number> = BufReader::new(f).lines()
		.filter_map(|l| l.ok())
		.map(|l| Number::parse(&mut l.chars()))
		.collect();

	println!("Part1: {}", part1(values.clone().into_iter()));
	println!("Part2: {}", part2(values));
}
