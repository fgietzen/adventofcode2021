use std::fs::File;
use std::io::{BufRead, BufReader};

mod cave;

fn part1(graph: &cave::Graph) -> usize {
	return graph.paths(false);
}

fn part2(graph: &cave::Graph) -> usize {
	return graph.paths(true);
}

fn main() {
	let f = File::open("inputs/day12_input.txt").expect("Could not find input file!");

	let mut graph_builder = cave::Graph::builder();
	BufReader::new(f)
		.lines()
		.filter_map(|l| l.ok())
		.map(|l| {
			let mut from_to = l.split("-");
			let from = from_to.next().unwrap().to_string();
			let to = from_to.next().unwrap().to_string();
			return (from, to);
		})
		.for_each(|(from, to)| graph_builder.add_edge(from, to));

	let graph = graph_builder.build();

	println!("Part1: {}", part1(&graph));
	println!("Part2: {}", part2(&graph));
}
