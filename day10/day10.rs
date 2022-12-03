use std::fs::File;
use std::io::{BufRead, BufReader};

mod syntax;

fn find_invalid_chars(line: &str) -> Option<char> {
	let mut stack: Vec<syntax::Chunk> = Vec::new();

	for c in line.chars() {
		if let Ok(chunk) = syntax::Chunk::try_from(c) {
			stack.push(chunk);
		} else {
			let opening = stack.pop().unwrap();
			if opening.closing_character() != c {
				return Some(c);
			}
		}
	}

	return None;
}

fn score1(c: char) -> u32 {
	return match c {
		')' => 3,
		']' => 57,
		'}' => 1197,
		'>' => 25137,
		_ => panic!("No score for character {}", c)
	};
}

fn part1<'a, T: Iterator<Item=&'a String>>(lines: T) -> u32 {
	return lines
		.filter_map(|l| find_invalid_chars(&l))
		.map(|c| score1(c))
		.sum();
}

fn complete_lines(line: &str) -> Option<Vec<char>> {
	let mut stack: Vec<syntax::Chunk> = Vec::new();

	for c in line.chars() {
		if let Ok(chunk) = syntax::Chunk::try_from(c) {
			stack.push(chunk);
		} else {
			let opening = stack.pop().unwrap();
			if opening.closing_character() != c {
				return None;
			}
		}
	}

	return Some(stack.iter().rev()
		.map(|chunk| chunk.closing_character())
		.collect());
}

fn char_value(c: &char) -> usize {
	return match c {
		')' => 1,
		']' => 2,
		'}' => 3,
		'>' => 4,
		_ => panic!("Invalid character")
	}
}

fn score2(remaining_chars: Vec<char>) -> usize {
	return remaining_chars.iter()
		.fold(0, |acc, c| acc * 5 + char_value(c));
}

fn part2<'a, T: Iterator<Item=&'a String>>(lines: T) -> usize {
	let mut scores: Vec<usize> = lines
		.filter_map(|l| complete_lines(&l))
		.map(|c| score2(c))
		.collect();
	scores.sort();

	return scores[scores.len()/2];
}

fn main() {
	let f = File::open("inputs/day10_input.txt")
		.expect("Could not find input file!");

	let lines: Vec<String> = BufReader::new(f).lines()
		.filter_map(|l| l.ok())
		.collect();

	println!("Part1: {}", part1(lines.iter()));
	println!("Part2: {}", part2(lines.iter()));
}
