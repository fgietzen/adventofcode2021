use std::cmp::{max, min};

pub type Point = (i64, i64, i64);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Instruction {
	On(Cuboid),
	Off(Cuboid)
}

impl Instruction {
	pub fn sign(&self) -> i64 {
		return match self {
			Instruction::On(_) => 1,
			Instruction::Off(_) => -1
		}
	}

	pub fn area(&self) -> &Cuboid {
		return match self {
			Instruction::On(c) => c,
			Instruction::Off(c) => c
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Cuboid {
	bottom_left_back: Point,
	top_right_front: Point
}

impl Cuboid {
	pub fn new(bottom_left_back: Point, top_right_front: Point) -> Self {
		return Cuboid { bottom_left_back, top_right_front };
	}

	pub fn from(&self) -> Point {
		return self.bottom_left_back;
	}

	pub fn to(&self) -> Point {
		return self.top_right_front;
	}

	pub fn volume(&self) -> u64 {
		return ((self.top_right_front.0 - self.bottom_left_back.0).abs() as u64 + 1)
			* ((self.top_right_front.1 - self.bottom_left_back.1).abs() as u64 + 1)
			* ((self.top_right_front.2 - self.bottom_left_back.2).abs() as u64 + 1);
	}

	pub fn intersect(&self, other: &Self) -> Option<Self> {
		let x0 = max(self.bottom_left_back.0, other.bottom_left_back.0);
		let y0 = max(self.bottom_left_back.1, other.bottom_left_back.1);
		let z0 = max(self.bottom_left_back.2, other.bottom_left_back.2);

		let x1 = min(self.top_right_front.0, other.top_right_front.0);
		let y1 = min(self.top_right_front.1, other.top_right_front.1);
		let z1 = min(self.top_right_front.2, other.top_right_front.2);

		if x1 < x0 || y1 < y0 || z1 < z0 {
			return None;
		}

		return Some(Cuboid::new((x0, y0, z0), (x1, y1, z1)));
	}
}

#[cfg(test)]
mod tests {
	use super::Cuboid;

	#[test]
	fn test_volume1() {
		let cube = Cuboid::new((-2, -2, -2), (2, 2, 2));

		assert_eq!(cube.volume(), 125);
	}

	#[test]
	fn test_volume2() {
		let cube = Cuboid::new((-10, -17, -47), (-14, -19, -49));

		assert_eq!(cube.volume(), 45);
	}

	#[test]
	fn test_intersect1() {
		let cube1 = Cuboid::new((-2, -2, -2), (1, 1, 1));
		let cube2 = Cuboid::new((-1, -1, -1), (2, 2, 2));

		let expected = Some(Cuboid::new((-1, -1, -1), (1, 1, 1)));
		assert_eq!(cube1.intersect(&cube2), expected);
		assert_eq!(cube2.intersect(&cube1), expected);
	}

	#[test]
	fn test_intersect2() {
		let cube1 = Cuboid::new((0, 1, 0), (1, 1, 0));
		let cube2 = Cuboid::new((0, 1, 0), (0, 2, 0));

		let expected = Some(Cuboid::new((0, 1, 0), (0, 1, 0)));
		assert_eq!(cube1.intersect(&cube2), expected);
		assert_eq!(cube2.intersect(&cube1), expected);
	}

	#[test]
	fn test_intersect3() {
		let cube1 = Cuboid::new((-5, -5, -5), (-3, -3, -3));
		let cube2 = Cuboid::new((-2, -5, 0), (0, 2, 0));

		assert_eq!(cube1.intersect(&cube2), None);
		assert_eq!(cube2.intersect(&cube1), None);
	}
}
