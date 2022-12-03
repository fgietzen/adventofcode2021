use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

mod reactor;
use reactor::Instruction;
use crate::reactor::Cuboid;

fn number_of_cubes<'a, I: Iterator<Item=&'a Instruction>>(instructions: I) -> i64 {
	let mut include_exclude: Vec<Instruction> = Vec::new();
	for i in instructions {
		let mut new_include_exclude = Vec::new();
		for a in include_exclude.iter() {
			if let Some(intersection) = i.area().intersect(a.area()) {
				if intersection.volume() != 0 {
					match a {
						Instruction::On(_) => new_include_exclude.push(Instruction::Off(intersection)),
						Instruction::Off(_) => new_include_exclude.push(Instruction::On(intersection))
					}
				}
			}
		}
		include_exclude.extend(new_include_exclude);

		if let Instruction::On(_) = i {
			include_exclude.push(i.clone());
		}
	}

	return include_exclude.iter()
		.map(|i| i.sign() * i.area().volume() as i64)
		.sum();
}

fn part1<'a, I: Iterator<Item=&'a Instruction>>(instructions: I) -> i64 {
	let valid_area = -50..=50;
	let filtered_points = instructions
		.filter(|i| [i.area().from(), i.area().to()].iter()
			.filter(|p| valid_area.contains(&p.0)
				&& valid_area.contains(&p.1)
				&& valid_area.contains(&p.2))
			.count() == 2
		);

	return number_of_cubes(filtered_points);
}

fn part2<'a, I: Iterator<Item=&'a Instruction>>(instructions: I) -> i64 {
	return number_of_cubes(instructions);
}

fn main() {
	let instruction_regex = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

	let f = File::open("inputs/day22_input.txt")
		.expect("Could not open file!");
	let instructions: Vec<Instruction> = BufReader::new(f).lines()
		.filter_map(|l| l.ok())
		.map(|l|  {
			let capture = instruction_regex.captures_iter(&l).next().unwrap();
			let x0 = capture[2].parse::<i64>().unwrap();
			let x1 = capture[3].parse::<i64>().unwrap();
			let y0 = capture[4].parse::<i64>().unwrap();
			let y1 = capture[5].parse::<i64>().unwrap();
			let z0 = capture[6].parse::<i64>().unwrap();
			let z1 = capture[7].parse::<i64>().unwrap();
			let area = Cuboid::new((x0, y0, z0), (x1, y1, z1));

			return match &capture[1] {
				"on" => Instruction::On(area),
				"off" => Instruction::Off(area),
				_ => panic!("Unexpected command!")
			}
		}).collect();

	println!("part1: {}", part1(instructions.iter()));
	println!("part2: {}", part2(instructions.iter()));
}
