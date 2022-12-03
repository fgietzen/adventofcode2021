use std::collections::HashSet;

type Point = (usize, usize);

#[derive(Debug, Clone)]
pub struct Octopuses {
	size_x: usize,
	size_y: usize,
	population: Vec<Vec<u32>>
}

impl Octopuses {
	fn neighbours(&self, (i, j): Point) -> Vec<Point> {
		return [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)].iter()
			.map(|&m| (m.0 + i as i64, m.1 + j as i64))
			.filter(|&(x, y)| x >= 0 && x < self.size_x as i64 && y >= 0 && y < self.size_y as i64)
			.map(|(x, y)| (x as usize, y as usize))
			.collect();
	}

	pub fn amount(&self) -> usize {
		return self.size_x * self.size_y;
	}

	pub fn step(&mut self) -> usize {
		let mut flashed = HashSet::new();

		for i in 0..self.size_x {
			for j in 0..self.size_y {
				self.activate(&mut flashed, (i, j));
			}
		}

		return flashed.len();
	}

	fn activate(&mut self, flashed: &mut HashSet<Point>, (i, j): Point) {
		if flashed.contains(&(i, j)) {
			return;
		}

		self.population[i][j] += 1;
		if self.population[i][j] > 9 {
			flashed.insert((i, j));
			self.population[i][j] = 0;

			for n in self.neighbours((i, j)) {
				self.activate(flashed, n);
			}
		}
	}
}

impl From<&str> for Octopuses {
	fn from(value: &str) -> Self {
		let population: Vec<Vec<u32>> = value.lines()
			.map(|l| l.chars()
				.map(|c| c as u32 - 48)
				.collect()
			).collect();
		let size_x = population.len();
		let size_y = population[0].len();

		return Octopuses { size_x, size_y, population };
	}
}
