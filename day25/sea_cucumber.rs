use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeaCucumber {
	EAST,
	SOUTH,
}

impl SeaCucumber {
	pub fn character(&self) -> char {
		return match self {
			SeaCucumber::EAST => '>',
			SeaCucumber::SOUTH => 'v'
		};
	}
}

impl From<char> for SeaCucumber {
	fn from(value: char) -> Self {
		return match value {
			'>' => SeaCucumber::EAST,
			'v' => SeaCucumber::SOUTH,
			c => panic!("Unexpected character: {}", c)
		};
	}
}

#[derive(Debug, Clone)]
pub struct Grid {
	size_x: usize,
	size_y: usize,
	values: Vec<Vec<Option<SeaCucumber>>>,
}

impl Grid {
	pub fn new(values: Vec<Vec<Option<SeaCucumber>>>) -> Self {
		let size_x = values.len();
		let size_y = values.iter().map(|row| row.len())
			.max().unwrap_or(0);

		return Grid { size_x, size_y, values };
	}
}

impl Display for Grid {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for row in self.values.iter() {
			for value in row {
				if let Some(cucumber) = value {
					write!(f, "{}", cucumber.character())?;
				} else {
					write!(f, ".")?;
				}
			}
			writeln!(f)?;
		}

		return Ok(());
	}
}

pub fn step(grid: Grid) -> (Grid, bool) {
	let mut moved = false;

	let mut new_grid = grid.clone();
	for i in 0..grid.size_x {
		for j in 0..grid.size_y {
			if let Some(cucumber) = &grid.values[i][j] {
				let y = (j + 1) % grid.size_y;
				if cucumber == &SeaCucumber::EAST && grid.values[i][y] == None {
					new_grid.values[i].swap(j, y);
					moved |= true;
				}
			}
		}
	}

	let grid = new_grid.clone();
	for i in 0..grid.size_x {
		for j in 0..grid.size_y {
			if let Some(cucumber) = &grid.values[i][j] {
				let x = (i + 1) % grid.size_x;
				if cucumber == &SeaCucumber::SOUTH && grid.values[x][j] == None {
					new_grid.values[i][j] = None;
					new_grid.values[x][j] = Some(SeaCucumber::SOUTH);
					moved |= true;
				}
			}
		}
	}

	return (new_grid, moved);
}
