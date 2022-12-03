use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub type Dot = (i32, i32);

pub fn parse_point(value: &str) -> Option<Dot> {
	let mut split = value.split(",");
	let x = split.next().map(|v| v.parse::<i32>().unwrap())?;
	let y = split.next().map(|v| v.parse::<i32>().unwrap())?;
	return Some((x, y));
}

pub enum Instruction {
	FoldHorizontal(i32),
	FoldVertical(i32)
}

impl TryFrom<String> for Instruction {
	type Error = ();

	fn try_from(value: String) -> Result<Self, Self::Error> {
		let mut split = value.split("=");
		let direction = split.next().unwrap();
		let value = split.next().unwrap().parse::<i32>().unwrap();
		return match direction {
			"x" => Ok(Instruction::FoldVertical(value)),
			"y" => Ok(Instruction::FoldHorizontal(value)),
			_ => Err(())
		}
	}
}

#[derive(Debug, Clone)]
pub struct Manual {
	dots: HashSet<Dot>
}

impl Manual {
	pub fn new(dots: HashSet<Dot>) -> Self {
		return Manual { dots };
	}

	pub fn number_dots(&self) -> usize {
		return self.dots.len();
	}

	pub fn fold(&self, instruction: &Instruction) -> Self {
		return match instruction {
			Instruction::FoldHorizontal(v) => self.fold_horizontal(*v),
			Instruction::FoldVertical(v) => self.fold_vertical(*v)
		}
	}

	fn fold_horizontal(&self, value: i32) -> Self {
		let dots_after_fold = self.dots.iter()
			.map(|&(x, y)| if y > value { (x, 2*value - y) } else { (x, y) })
			.collect();

		return Manual { dots: dots_after_fold };
	}

	fn fold_vertical(&self, value: i32) -> Self {
		let dots_after_fold = self.dots.iter()
			.map(|&(x, y)| if x > value { (2*value - x, y) } else { (x, y) })
			.collect();

		return Manual { dots: dots_after_fold };
	}
}

impl Display for Manual {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let max_column = self.dots.iter()
			.map(|(x, _)| *x)
			.max().unwrap_or(0);
		let max_row = self.dots.iter()
			.map(|(_, y)| *y)
			.max().unwrap_or(0);

		let manual_string = (0..=max_row)
			.map(|y| (0..=max_column)
				.fold(String::new(), |acc, x|
					acc + if self.dots.contains(&(x, y)) { "#" } else { "." }
				)
			).fold(String::new(), |acc, line| acc + &line + "\n");

		return write!(f, "{}", manual_string);
	}
}
