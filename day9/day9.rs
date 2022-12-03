mod basin;

fn part1(heightmap: &basin::Heightmap) -> u32 {
	return heightmap.valleys()
		.map(|v| heightmap.value(v) + 1)
		.sum::<u32>();
}

fn part2(heightmap: &basin::Heightmap) -> usize {
	let mut size_of_basins: Vec<usize> = heightmap.valleys()
		.map(|v| heightmap.basin(v).len())
		.collect();
	size_of_basins.sort();

	return size_of_basins.iter().rev()
		.take(3)
		.product();
}

fn main() {
	let content = std::fs::read_to_string("inputs/day9_input.txt")
		.expect("Could not read file content!");

	let heightmap = basin::Heightmap::try_from(content.as_str())
		.expect("Could not create heightmap!");

	println!("Part1: {}", part1(&heightmap));
	println!("Part2: {}", part2(&heightmap));
}
