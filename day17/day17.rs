use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::ops::{RangeInclusive};

use regex::Regex;

fn pos(x_0: i32, y_0: i32, s: i32) -> (i32, i32) {
	let sum_to_n = |n| n*(n+1)/2;
	let d = min(s, x_0);
	return (x_0*(d+1) - sum_to_n(d), y_0*(s+1) - sum_to_n(s));
}

fn part1(x_range: &RangeInclusive<i32>, y_range: &RangeInclusive<i32>) -> i32 {
	let mut max_height = 0;
	for x in 0..*x_range.end() {
		for y in *y_range.start()..-*y_range.start() {
			let (pos_x, pos_y) = pos(x, y, 2*y+1);
			if x_range.contains(&pos_x) && y_range.contains(&pos_y) {
				let max_y = pos(pos_x, y, y-1).1;
				max_height = max(max_height, max_y);
			}
		}
	}

	return max_height;
}

fn part2(x_range: &RangeInclusive<i32>, y_range: &RangeInclusive<i32>) -> usize {
	let mut x_to_s = HashMap::new();
	for x in 0..*x_range.end()+1 {
		for s in 0..1000 {
			let pos_x = pos(x, 0, s).0;
			if x_range.contains(&pos_x) {
				x_to_s.entry(x).or_insert(Vec::new()).push(s);
			}
		}
	}

	let mut s_to_y = HashMap::new();
	for y in *y_range.start()..-*y_range.start() {
		for s in 0..1000 {
			let pos_y = pos(0, y, s).1;
			if y_range.contains(&pos_y) {
				s_to_y.entry(s).or_insert(Vec::new()).push(y);
			}
		}
	}

	let velocities: HashSet<(i32, i32)> = x_to_s.iter()
		.map(|(x, ss)| ss.iter()
			.map(|s| (s, s_to_y.get(s)))
			.filter(|(_, v)| v.is_some())
			.map(|(s, v)| (s, v.unwrap()))
			.map(|(s, ys)| ys.iter()
				.filter(|&y| {
					let (pos_x, pos_y) = pos(*x, *y, *s);
					return x_range.contains(&pos_x) && y_range.contains(&pos_y);
				})
			).flatten()
			.map(|y| (*x, *y))
		).flatten()
		.collect();

	return velocities.len();
}

fn main() {
	let input_re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();

	let content = std::fs::read_to_string("inputs/day17_input.txt")
		.expect("Could not read file!");

	let capture = input_re.captures_iter(&content).next().unwrap();
	let x_range = capture[1].parse::<i32>().unwrap()..=capture[2].parse::<i32>().unwrap();
	let y_range = capture[3].parse::<i32>().unwrap()..=capture[4].parse::<i32>().unwrap();

	println!("Part1: {}", part1(&x_range, &y_range));
	println!("Part2: {}", part2(&x_range, &y_range));
}
