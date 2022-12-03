fn part1(crabs: &Vec<u32>) -> u32 {
	let median = crabs[crabs.len() / 2];
	return crabs.iter()
		.map(|v| (*v as i32 - median as i32).abs() as u32)
		.sum();
}

fn part2(crabs: &Vec<u32>) -> u32 {
	let min = *crabs.iter().min().unwrap();
	let max = *crabs.iter().max().unwrap();

	return (min..=max)
		.map(|i| crabs.iter()
			.map(|v| (*v as i32 - i as i32).abs() as u32)
			.map(|d| d*(d+1)/2)
			.sum()
		).min().unwrap();
}

fn main() {
	let content = std::fs::read_to_string("inputs/day7_input.txt")
		.expect("Could not read file content!");
	let mut crabs: Vec<u32> = content
		.strip_suffix("\n").unwrap_or(&content)
		.split(",")
		.map(|v| v.parse::<u32>().unwrap())
		.collect();
	crabs.sort();

	println!("Part1: {}", part1(&crabs));
	println!("Part2, {}", part2(&crabs));
}
