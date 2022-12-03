use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

mod dice;
use dice::{DeterministicDice, DiracDiceGame};

fn part1(mut pos_player1: u64, mut pos_player2: u64) -> u64 {
	let mut score_player1 = 0;
	let mut score_player2 = 0;

	let mut dice = DeterministicDice::new();
	loop {
		pos_player1 = (pos_player1 + dice.roll() + dice.roll() + dice.roll()) % 10;
		score_player1 += pos_player1 + 1;

		if score_player1 >= 1000 {
			break;
		}

		pos_player2 = (pos_player2 + dice.roll() + dice.roll() + dice.roll()) % 10;
		score_player2 += pos_player2 + 1;

		if score_player2 >= 1000 {
			break;
		}
	}

	return dice.number_of_rolls() * min(score_player1, score_player2);
}

fn part2(pos_player1: u64, pos_player2: u64) -> u64 {
	let mut game = DiracDiceGame::new();
	let (wins1, wins2) = game.play(pos_player1, pos_player2);
	return max(wins1, wins2);
}

fn main() {
	let f = File::open("inputs/day21_input.txt")
		.expect("Could not open file!");
	let mut lines = BufReader::new(f).lines()
		.map(|l| l.ok());
	let mut next_line = || lines.next().unwrap().unwrap();

	let pos_player1 = next_line().pop().unwrap().to_digit(10).unwrap() as u64 - 1;
	let pos_player2 = next_line().pop().unwrap().to_digit(10).unwrap() as u64 - 1;

	println!("Part1: {}", part1(pos_player1, pos_player2));
	println!("Part2: {}", part2(pos_player1, pos_player2));
}
