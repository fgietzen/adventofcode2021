use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod chitons;
use crate::chitons::{Coordinate, Node};

fn neighbours((i, j): Coordinate, size_x: usize, size_y: usize) -> impl Iterator<Item=Coordinate> {
	return [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
		.map(move |&(mx, my)| (mx + i as isize, my + j as isize))
		.filter(move |&(x, y)| x >= 0 && x < size_x as isize && y >= 0 && y < size_y as isize)
		.map(|(x, y)| (x as usize, y as usize));
}

fn dijkstra(cost_matrix: &Vec<Vec<u8>>) -> u64 {
	let size_x = cost_matrix.len();
	let size_y = cost_matrix[0].len();

	let mut distances: Vec<Vec<u64>> = (0..size_x).map(|_| (0..size_y).map(|_| u64::MAX).collect()).collect();
	distances[0][0] = 0;

	let mut queue: BinaryHeap<Node> = BinaryHeap::new();
	queue.push(Node::new((0, 0)).with_distance(0));

	while let Some(node) = queue.pop() {
		if node.coordinates() == (size_x-1, size_y-1) {
			return node.distance();
		}

		let (x, y) = node.coordinates();
		if node.distance() > distances[x][y] {
			continue;
		}

		for (nx, ny) in neighbours(node.coordinates(), size_x, size_y) {
			let distance = node.distance() + cost_matrix[nx][ny] as u64;
			if distance < distances[nx][ny] {
				queue.push(Node::new((nx, ny)).with_distance(distance));
				distances[nx][ny] = distance;
			}
		}
	}

	panic!("Could not find exit!");
}

fn part1(cost_matrix: &Vec<Vec<u8>>) -> u64 {
	return dijkstra(cost_matrix);
}

fn part2(cost_matrix: &Vec<Vec<u8>>) -> u64 {
	let size_x = cost_matrix.len();
	let size_y = cost_matrix[0].len();

	let expanded_cost_matrix = (0..(5*size_x))
		.map(|x| (0..(5*size_y))
			.map(|y| {
				let cost = cost_matrix[x % size_x][y % size_y] + (x/size_x + y/size_y) as u8;
				return if cost < 10 { cost } else { cost - 9 };
			}).collect::<Vec<_>>()
		).collect::<Vec<_>>();

	return dijkstra(&expanded_cost_matrix);
}

fn main() {
	let f = File::open("inputs/day15_input.txt").expect("Could not open file!");

	let cost_matrix: Vec<Vec<u8>> = BufReader::new(f).lines()
		.filter_map(|l| l.ok())
		.map(|l| l.chars()
			.map(|c| c as u8 - 48)
			.collect()
		).collect();

	println!("Part1: {}", part1(&cost_matrix));
	println!("Part2: {}", part2(&cost_matrix));
}