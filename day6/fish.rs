pub struct Population {
	pop: [u64; 9]
}

impl Population {
	pub fn step(&mut self) {
		let resetted = self.pop[0];
		self.pop.rotate_left(1);
		self.pop[6] += resetted;
	}

	pub fn size(&self) -> u64 {
		return self.pop.iter().sum();
	}
}

impl<'a, T: Iterator<Item=&'a usize>> From<T> for Population {
	fn from(fish: T) -> Self {
		let mut pop = [0; 9];
		for f in fish {
			pop[*f] += 1;
		}

		return Population { pop };
	}
}
