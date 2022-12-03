#[derive(Debug, Copy, Clone)]
struct CellValue {
	marked: bool,
	value: u32
}

impl CellValue {
	fn new(v: u32) -> Self {
		return CellValue { marked: false, value: v }
	}
}

impl Default for CellValue {
	fn default() -> Self {
		return CellValue { marked: false, value: 0 };
	}
}

pub struct BoardBuilder<const N: usize> {
	row_index: usize,
	grid: [[CellValue; N]; N]
}

impl<const N: usize> BoardBuilder<N> {
	fn new() -> Self {
		return BoardBuilder { row_index: 0, grid: [[CellValue::default(); N]; N] };
	}

	pub fn add_row(&mut self, l: &str) {
		for (j, v) in l.split_whitespace().enumerate() {
			let value = v.parse::<u32>().unwrap();
			self.grid[self.row_index][j] = CellValue::new(value);
		}
		self.row_index += 1;
	}

	pub fn build(&self) -> Board<N> {
		return Board { grid: self.grid };
	}
}

#[derive(Debug, Clone)]
pub struct Board<const N: usize> {
	grid: [[CellValue; N]; N]
}

impl<const N: usize> Board<N> {
	pub fn builder() -> BoardBuilder<N> {
		return BoardBuilder::new();
	}

	pub fn mark(&mut self, v: u32) {
		self.grid.iter_mut()
			.flat_map(|l| l.iter_mut())
			.filter(|cell| cell.value == v)
			.for_each(|cell| cell.marked = true);
	}

	pub fn has_won(&self) -> bool {
		for i in 0..self.grid.len() {
			let mut row_full = true;
			let mut column_full = true;
			for j in 0..self.grid[i].len() {
				row_full &= self.grid[i][j].marked;
				column_full &= self.grid[j][i].marked;
			}

			if row_full || column_full {
				return true;
			}
		}

		return false;
	}

	pub fn sum_of_unmarked(&self) -> u32 {
		return self.grid.iter()
			.flat_map(|l| l.iter())
			.filter(|c| !c.marked)
			.map(|c| c.value)
			.sum();
	}
}
