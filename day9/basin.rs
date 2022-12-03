use std::collections::HashSet;

pub struct Heightmap {
	size_x: usize,
	size_y: usize,
	map: Vec<Vec<u32>>
}

type Point = (usize, usize);
const MAX_HEIGHT: u32 = 9;

impl Heightmap {
	pub fn value(&self, (i, j): Point) -> u32 {
		return self.map[i][j];
	}

	pub fn valleys(&self) -> impl Iterator<Item = Point> + '_ {
		return (0..self.map.len())
			.map(|i| (0..self.map[i].len())
				.map(move |j| (i, j))
				.filter_map(|p| if self.is_valley(p) { Some(p) } else { None })
			).flatten();
	}

	pub fn basin(&self, p: Point) -> HashSet<Point> {
		let mut points: HashSet<Point> = HashSet::new();

		self.explore_basin(&mut points, p);
		return points;
	}

	fn neighbours(&self, (i, j): Point) -> Vec<Point> {
		return [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
			.map(|&m| (m.0 + i as i32, m.1 + j as i32))
			.filter(|&(x, y)| x >= 0 && x < self.size_x as i32 && y >= 0 && y < self.size_y as i32)
			.map(|(x, y)| (x as usize, y as usize))
			.collect();
	}

	fn is_valley(&self, (i, j): Point) -> bool {
		let mut valley = true;
		for neighbour in self.neighbours((i, j)) {
			valley &= self.map[i][j] < self.map[neighbour.0][neighbour.1];
		}

		return valley;
	}

	fn explore_basin(&self, points: &mut HashSet<Point>, (i, j): Point) {
		if points.contains(&(i, j)) || self.map[i][j] >= MAX_HEIGHT {
			return;
		}

		points.insert((i, j));

		for neighbour in self.neighbours((i, j)) {
			self.explore_basin(points, neighbour);
		}
	}
}

impl TryFrom<&str> for Heightmap {
	type Error = &'static str;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		let map: Vec<Vec<u32>> = s
			.lines()
			.map(|l| l.chars()
					.map(|c| c as u32 - 48)
					.collect()
			).collect();

		let size_x = map.len();
		let size_y = map.get(0).map(|v| v.len()).unwrap_or(0);

		return Ok(Heightmap { map, size_x, size_y });
	}
}

#[cfg(test)]
mod tests {
	use crate::basin::Heightmap;

	#[test]
	fn test_neighbours() {
		let map = Heightmap {
			size_x: 3,
			size_y: 3,
			map: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
		};

		let n = map.neighbours((0, 1));

		assert!(n.contains(&(0, 0)));
		assert!(n.contains(&(1, 1)));
		assert!(n.contains(&(0, 2)));
	}
}
