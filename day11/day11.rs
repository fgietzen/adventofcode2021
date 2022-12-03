mod cavern;

fn part1(mut octopuses: cavern::Octopuses) -> usize {
	return (0..100)
		.map(|_| octopuses.step())
		.sum();
}

fn part2(mut octopuses: cavern::Octopuses) -> usize {
	let mut step_number = 1;
	while octopuses.step() != octopuses.amount() {
		step_number += 1;
	}

	return step_number;
}

fn main() {
	let content = std::fs::read_to_string("inputs/day11_input.txt")
		.expect("Could not read file content!");

	let octopuses = cavern::Octopuses::from(content.as_str());

	println!("Part1: {}", part1(octopuses.clone()));
	println!("Part2: {}", part2(octopuses));
}
