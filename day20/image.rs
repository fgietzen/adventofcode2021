use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub type Point = (i32, i32);

#[derive(Debug, Clone)]
pub struct InfinityCanvas{
	all_true: bool,
	min_x: i32,
	max_x: i32,
	min_y: i32,
	max_y: i32,
	pub points: HashSet<Point>
}

impl InfinityCanvas {
	pub fn new(points: HashSet<Point>) -> InfinityCanvas {
		let min_x = points.iter()
			.map(|(i, _)| *i)
			.min().unwrap();
		let max_x = points.iter()
			.map(|(i, _)| *i)
			.max().unwrap();
		let min_y = points.iter()
			.map(|(_, j)| *j)
			.min().unwrap();
		let max_y = points.iter()
			.map(|(_, j)| *j)
			.max().unwrap();

		return InfinityCanvas { all_true: false, min_x, max_x, min_y, max_y, points };
	}

	pub fn len(&self) -> usize {
		return self.points.len();
	}

 	pub fn apply_enhancing(self, enhancement_algorithm: &Vec<bool>) -> Self {
		let mut new_points = HashSet::with_capacity(self.points.len());

		for i in self.min_x-1..=self.max_x+1  {
			for j in self.min_y-1..=self.max_y+1  {
				if self.pixel_value(enhancement_algorithm, (i, j)) {
					new_points.insert((i, j));
				}
			}
		}

		return InfinityCanvas {
			all_true: !self.all_true,
			min_x: self.min_x-1, max_x: self.max_x+1,
			min_y: self.min_y-1, max_y: self.max_y+1,
			points: new_points
		};
	}

	fn pixel_value(&self, enhancement_algorithm: &Vec<bool>, (i, j): Point) -> bool {
		let value = [
			(i-1, j-1), (i-1, j), (i-1, j+1),
			(i, j-1), (i, j), (i, j+1),
			(i+1, j-1), (i+1, j), (i+1, j+1)
		].iter().rev()
			.enumerate()
			.fold(0, |acc, (i, p)| {
				if p.0 < self.min_x || p.0 > self.max_x || p.1 < self.min_y || p.1 > self.max_y {
					return acc + if self.all_true { 2_usize.pow(i as u32) } else { 0 };
				}
				return acc + if self.points.contains(p) { 2_usize.pow(i as u32) } else { 0 };
			});

		return *enhancement_algorithm.get(value)
			.expect("Value could not be found in algorithm");
	}
}

impl Display for InfinityCanvas {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for i in self.min_x..=self.max_x {
			for j in self.min_y..=self.max_y {
				write!(f, "{}", if self.points.contains(&(i, j)) { "#" } else { "."})?;
			}
			write!(f, "\n")?;
		}
		return Ok(());
	}
}
