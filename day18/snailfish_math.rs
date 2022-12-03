use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use std::str::Chars;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Number {
	Literal(u32),
	Pair(Box<Number>, Box<Number>),
}

impl Number {
	pub fn parse(chars: &mut Chars) -> Number {
		return match chars.next() {
			Some('[') => {
				let left = Number::parse(chars);
				assert!(chars.next().map(|c| c == ',').unwrap_or(false));
				let right = Number::parse(chars);
				assert!(chars.next().map(|c| c == ']').unwrap_or(false));

				return Number::Pair(Box::new(left), Box::new(right));
			},
			Some(a) => Number::Literal(a.to_digit(10).expect("Not a valid number!")),
			None => panic!("Unexpected EOL")
		}
	}

	pub fn magnitude(&self) -> u64 {
		return match self {
			Number::Literal(v) => *v as u64,
			Number::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude()
		};
	}

	pub fn reduce(self) -> Self {
		let mut number = self;

		loop {
			loop {
				let (exploded, _, res) = number.explode(0);
				number = exploded;
				if !res {
					break;
				}
			}

			let (split, res) = number.split();
			number = split;
			if !res {
				break;
			}
		}

		return number;
	}

	fn explode(self, height: usize) -> (Number, (u32, u32), bool) {
		return match self {
			Number::Literal(_) => (self, (0, 0), false),
			Number::Pair(left, right) => match (*left, *right) {
				(Number::Literal(l), Number::Literal(r)) if height >= 4 => {
					(Number::Literal(0), (l, r), true)
				}
				(left, right) => {
					let (left_child, (l_l, l_r), exploded) = left.explode(height + 1);
					if exploded {
						let right_exploded = if l_r == 0 { right } else { right.insert_left(l_r) };
						return (Number::Pair(
							Box::new(left_child),
							Box::new(right_exploded),
						), (l_l, 0), true);
					}

					let (right_child, (r_l, r_r), exploded) = right.explode(height + 1);
					if exploded {
						let left_exploded = if r_l == 0 { left_child } else { left_child.insert_right(r_l) };
						return (Number::Pair(
							Box::new(left_exploded),
							Box::new(right_child)
						), (0, r_r), true)
					}

					return (Number::Pair(
						Box::new(left_child),
						Box::new(right_child)
					), (0, 0), false)
				}
			}
		};
	}

	fn split(self) -> (Self, bool) {
		return match self {
			Number::Literal(v) if v >= 10 => (Number::Pair(
				Box::new(Number::Literal(v/2)),
				Box::new(Number::Literal(if v % 2 == 0 { v/2 } else { v/2+1 })),
			), true),
			Number::Literal(_) => (self, false),
			Number::Pair(l, r) => {
				let (left_split, was_split) = l.split();
				if was_split {
					return (Number::Pair(Box::new(left_split), r), true);
				}

				let (right_split, was_split) = r.split();
				return (Number::Pair(Box::new(left_split), Box::new(right_split)), was_split);
			}
		}
	}

	fn insert_left(self, v: u32) -> Self {
		return match self {
			Number::Literal(l) => Number::Literal(l + v),
			Number::Pair(l, r) => Number::Pair(
				Box::new(l.insert_left(v)), r,
			)
		};
	}

	fn insert_right(self, v: u32) -> Self {
		return match self {
			Number::Literal(l) => Number::Literal(l + v),
			Number::Pair(l, r) => Number::Pair(
				l, Box::new(r.insert_right(v)),
			)
		};
	}
}

impl Add for Number {
	type Output = Number;

	fn add(self, rhs: Self) -> Self::Output {
		return Number::Pair(Box::new(self), Box::new(rhs)).reduce();
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		return match self {
			Number::Literal(v) => write!(f, "{}", v),
			Number::Pair(l, r) => write!(f, "[{},{}]", l, r)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Number;

	fn pair(left: u32, right: u32) -> Number {
		return Number::Pair(
			Box::new(Number::Literal(left)),
			Box::new(Number::Literal(right)),
		);
	}

	#[test]
	fn test_parse() {
		let exp = "[[1,[5,5]],[4,6]]";

		let n = Number::parse(&mut exp.chars());

		assert_eq!(n, (Number::Literal(1)
			+ pair(5,5))
			+ pair(4, 6)
		);
	}

	#[test]
	fn test_display() {
		let n = (Number::Literal(1) + pair(5, 5)) + pair(4, 6);

		assert_eq!(&n.to_string(), "[[1,[5,5]],[4,6]]")
	}

	#[test]
	fn test_add() {
		let n1 = pair(4, 6);
		let n2 = pair(5, 4);

		if let Number::Pair(l, r) = n1 + n2 {
			assert_eq!(l.as_ref(), &pair(4, 6));
			assert_eq!(r.as_ref(), &pair(5, 4));
		} else {
			panic!("Expected pair!");
		}
	}

	#[test]
	fn test_magnitude() {
		let n = pair(4, 6) +
			pair(5, 4);

		let magnitude = n.magnitude();
		assert_eq!(magnitude, 3 * (3 * 4 + 2 * 6) + 2 * (3 * 5 + 2 * 4));
	}

	#[test]
	fn test_explode1() {
		let n = pair(9, 8)
			+ Number::Literal(1)
			+ Number::Literal(2)
			+ Number::Literal(3)
			+ Number::Literal(4);

		let exploded = n.explode(0).0;

		assert_eq!(exploded, pair(0, 9)
			+ Number::Literal(2)
			+ Number::Literal(3)
			+ Number::Literal(4)
		)
	}

	#[test]
	fn test_explode2() {
		let n = Number::Literal(7)
			+ (Number::Literal(6)
			+ (Number::Literal(5)
			+ (Number::Literal(4)
			+ pair(3, 2))));

		let exploded = n.explode(0).0;

		assert_eq!(exploded, Number::Literal(7)
			+ (Number::Literal(6)
			+ (Number::Literal(5)
			+ pair(7, 0)))
		)
	}

	#[test]
	fn test_explode3() {
		let n = (Number::Literal(6)
			+ (Number::Literal(5)
			+ (Number::Literal(4)
			+ pair(3, 2))))
			+ Number::Literal(1);

		let reduced = n.explode(0).0;

		assert_eq!(reduced, (Number::Literal(6)
			+ (Number::Literal(5)
			+ pair(7, 0)))
			+ Number::Literal(3)
		)
	}

	#[test]
	fn test_split() {
		let n = Number::Literal(12);

		let split = n.split().0;

		assert_eq!(split, pair(6, 6));
	}
}
