use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Polymer {
	elements: [char; 2]
}

impl Polymer {
	pub fn new(element1: char, element2: char) -> Self {
		return Polymer { elements: [element1, element2] };
	}

	pub fn morph(&self, element: char) -> [Polymer; 2] {
		return [
			Polymer::new(self.elements[0], element),
			Polymer::new(element, self.elements[1])
		];
	}
}

pub struct Instruction {
	rule: Polymer,
	result: char
}

impl Instruction {
	pub fn rule(&self) -> &Polymer {
		return &self.rule;
	}

	pub fn result(&self) -> char {
		return self.result;
	}
}

impl TryFrom<&str> for Instruction {
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let mut split = value.split(" -> ");
		let rule = split.next().map(|p| {
			let mut chars = p.chars();
			return Polymer::new(chars.next().unwrap(), chars.next().unwrap());
		}).ok_or("Could not parse rule")?;
		let result = split.next().map(|p| p.chars().next().unwrap())
			.ok_or("Could not parse result")?;

		return Ok(Instruction { rule, result})
	}
}

#[derive(Debug, Clone)]
pub struct Counter<T: Eq + Hash> {
	count: HashMap<T, i64>
}

impl<T: Eq + Hash> Counter<T> {
	pub fn new() -> Self {
		return Counter { count: HashMap::new() };
	}

	pub fn add(&mut self, value: T, amount: i64) {
		*self.count.entry(value).or_insert(0) += amount;
	}

	pub fn minus(&mut self, value: T, amount: i64) {
		*self.count.entry(value).or_insert(0) -= amount;
	}

	pub fn count(&self, value: &T) -> i64 {
		return self.count.get(value).map(|v| *v).unwrap_or(0);
	}

	pub fn max(&self) -> Option<(&T, i64)> {
		return self.count.iter()
			.max_by_key(|(_, &value)| value)
			.map(|(k, &v)| (k, v));
	}

	pub fn min(&self) -> Option<(&T, i64)> {
		return self.count.iter()
			.min_by_key(|(_, &value)| value)
			.map(|(k, &v)| (k, v));
	}
}
