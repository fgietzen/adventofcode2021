use std::cmp::Ordering;

pub type Coordinate = (usize, usize);

#[derive(Debug)]
pub struct Node {
	distance: u64,
	coordinate: Coordinate
}

impl Node {
	pub fn new(coordinate: Coordinate) -> Self {
		return Node { coordinate, distance: u64::MAX };
	}

	pub fn coordinates(&self) -> Coordinate {
		return self.coordinate;
	}

	pub fn distance(&self) -> u64 {
		return self.distance;
	}

	pub fn with_distance(mut self, distance: u64) -> Self {
		self.distance = distance;
		return self;
	}
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
	fn eq(&self, other: &Self) -> bool {
		return self.distance == other.distance;
	}
}

impl PartialOrd<Self> for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		return Some(self.cmp(&other));
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		return other.distance.cmp(&self.distance);
	}
}
