use std::fs::File;
use std::io::{BufRead, BufReader};

mod bingo;

const BOARD_SIZE: usize = 5;

fn part1(random_numbers: &Vec<u32>, mut boards: Vec<bingo::Board<BOARD_SIZE>>) -> u32 {
	for number in random_numbers {
		for board in boards.iter_mut() {
			board.mark(*number);

			if board.has_won() {
				return number * board.sum_of_unmarked();
			}
		}
	}

	panic!("No winner found!");
}

fn part2(random_numbers: &Vec<u32>, mut boards: Vec<bingo::Board<BOARD_SIZE>>) -> u32 {
	let mut res = None;
	for number in random_numbers {
		for board in boards.iter_mut() {
			board.mark(*number);
		}

		if boards.len() == 1 {
			res = boards.pop().map(|b| (b, number));
		} else {
			boards.retain(|b| !b.has_won());
		}
	}

	if let Some((board, number)) = res {
		return number * board.sum_of_unmarked();
	}

	panic!("Worst board not found!");
}

fn main() {
	let f = File::open("inputs/day4_input.txt").expect("Could not open file!");
	let mut lines = BufReader::new(f).lines();

	let random_numbers: Vec<u32> = lines.next().unwrap().unwrap()
		.split(",")
		.map(|s| s.parse::<u32>().unwrap())
		.collect();
	let lines = lines
		.skip_while(|l| l.as_ref().unwrap().is_empty());

	let mut boards = Vec::new();

	let mut board_builder = bingo::Board::builder();
	for l in lines {
		let line = l.unwrap();

		if line.is_empty() {
			boards.push(board_builder.build());
			board_builder = bingo::Board::builder();
		} else {
			board_builder.add_row(&line);
		}
	}

	println!("Part1: {}", part1(&random_numbers, boards.clone()));
	println!("Part2: {}", part2(&random_numbers, boards));
}
