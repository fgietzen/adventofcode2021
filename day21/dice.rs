pub struct DeterministicDice {
	rolls: u64
}

impl DeterministicDice {
	pub fn new() -> Self {
		return DeterministicDice { rolls: 0 };
	}

	pub fn roll(&mut self) -> u64 {
		let value = (self.rolls % 100) + 1;
		self.rolls += 1;
		return value;
	}

	pub fn number_of_rolls(&self) -> u64 {
		return self.rolls;
	}
}

pub struct DiracDiceGame {
	cache: lru::LruCache<(u64, u64, u64, u64), (u64, u64)>
}

impl DiracDiceGame {
	pub fn new() -> Self {
		return DiracDiceGame { cache: lru::LruCache::unbounded() };
	}

	pub fn play(&mut self, pos_player1: u64, pos_player2: u64) -> (u64, u64) {
		return self.step(pos_player1, 0, pos_player2, 0);
	}

	fn step(&mut self,
			pos_player_turn: u64, score_player1: u64,
			pos_player_wait: u64, score_player2: u64
	) -> (u64, u64) {
		let cache_result = self.cache.get(&(pos_player_turn, pos_player_wait, score_player1, score_player2));
		if let Some(&wins) = cache_result {
			return wins;
		}

		if score_player2 >= 21 {
			return (0, 1);
		}

		// (sum of three rolls, quantity)
		let wins = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)].into_iter()
			.map(|(v, c)| {
				let new_pos = (pos_player_turn + v) % 10;
				let new_score = score_player1 + new_pos + 1;
				let (win2, win1) = self.step(pos_player_wait, score_player2, new_pos, new_score);

				(c * win1, c * win2)
			}).fold((0, 0), |(acc1, acc2), (w1, w2)| (acc1 + w1, acc2 + w2));

		self.cache.put((pos_player_turn, pos_player_wait, score_player1, score_player2), wins);
		return wins;
	}
}
