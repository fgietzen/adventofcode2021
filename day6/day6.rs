mod fish;

fn part1<'a, T: Iterator<Item=&'a usize>>(fish: T) -> u64 {
	let mut population = fish::Population::from(fish);
	for _ in 0..80 {
		population.step();
	}

	return population.size();
}

fn part2<'a, T: Iterator<Item=&'a usize>>(fish: T) -> u64 {
	let mut population = fish::Population::from(fish);
	for _ in 0..256 {
		population.step();
	}

	return population.size();
}

fn main() {
	let content = std::fs::read_to_string("inputs/day6_input.txt").expect("Could not read file content");
	let fish: Vec<usize> = content
		.strip_suffix("\n").unwrap_or(&content)
		.split(",")
		.map(|v| v.parse::<usize>().unwrap())
		.collect();

	println!("{}", part1(fish.iter()));
	println!("{}", part2(fish.iter()));
}