use std::fs::File;
use std::io::{BufRead, BufReader};

mod criteria;

fn get_bit(v: u32, i: u32) -> u32 {
	return v >> i & 1;
}

fn part1(values: &Vec<u32>, length: u32) -> u32 {
	let calculate_value = |c: criteria::Criteria| {
		return {0..length}
			.map(|i| (i, values.iter()
				.fold(0 as i32, |acc, x| acc + if get_bit(*x, i) == 1 { 1 } else { -1 }))
			).fold(0, |acc, (i, x)| acc | if c.pred(x) { 1 << i } else { 0 }) as u32;
	};

	let gamma = calculate_value(criteria::Criteria::MostCommon);
	let epsilon = calculate_value(criteria::Criteria::LeastCommon);

	return gamma * epsilon;
}

fn part2(values: &Vec<u32>, length: u32) -> u32 {
	let calculate_value = |c: criteria::Criteria| {
		let mut values = values.clone();
		for i in 0..length {
			let x = values.iter()
				.fold(0, |acc, x| acc + if get_bit(*x, length - i - 1) == 1 { 1 } else { -1 });
			let bit = if c.pred(x) { 1 } else { 0 };

			values.retain(|v| get_bit(*v, length - i - 1) == bit);

			if values.len() == 1 {
				return *values.get(0).unwrap();
			}
		}

		panic!("Could not find value!");
	};

	let o2_gen_rating = calculate_value(criteria::Criteria::MostCommon);
	let c02_scrubber_rating = calculate_value(criteria::Criteria::LeastCommon);

	return o2_gen_rating * c02_scrubber_rating;
}

fn main() {
	let f = File::open("inputs/day3_input.txt").expect("Could not open file!");

	let values: Vec<u32> = BufReader::new(f).lines()
		.map(|l| u32::from_str_radix(&l.unwrap(), 2).unwrap())
		.collect();

	let length = values.iter()
		.map(|l| (*l as f64).log2() as u32 + 1)
		.max().unwrap_or(0);

	println!("Part1: {}", part1(&values, length));
	println!("Part2: {}", part2(&values, length));
}
