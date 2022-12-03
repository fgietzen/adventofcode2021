pub enum Criteria {
	MostCommon,
	LeastCommon
}

impl Criteria {
	pub fn pred(&self, x: i32) -> bool {
		return match self {
			Criteria::MostCommon => x >= 0,
			Criteria::LeastCommon => x < 0
		};
	}
}
