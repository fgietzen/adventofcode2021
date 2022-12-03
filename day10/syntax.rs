#[derive(Debug, Clone, Copy)]
pub enum Chunk {
	Parenthesis,
	SquareBrackets,
	CurlyBrackets,
	TriangularBrackets,
}

impl Chunk {
	pub fn opening_character(&self) -> char {
		return match self {
			Chunk::Parenthesis => '(',
			Chunk::SquareBrackets => '[',
			Chunk::CurlyBrackets => '{',
			Chunk::TriangularBrackets => '<'
		}
	}

	pub fn closing_character(&self) -> char {
		return match self {
			Chunk::Parenthesis => ')',
			Chunk::SquareBrackets => ']',
			Chunk::CurlyBrackets => '}',
			Chunk::TriangularBrackets => '>'
		}
	}

	fn values() -> impl Iterator<Item=Chunk> {
		return [Chunk::Parenthesis, Chunk::SquareBrackets, Chunk::CurlyBrackets, Chunk::TriangularBrackets].into_iter();
	}
}

impl TryFrom<char> for Chunk {
	type Error = String;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		return Chunk::values()
			.map(|chunk| (chunk, chunk.opening_character()))
			.filter(|(_, opening_character)| value == *opening_character)
			.next().map(|(chunk, _)| chunk)
			.ok_or(format!("Invalid character: {}", value));
	}
}
