pub struct BitReader {
	bits: Vec<u8>,
	pos: usize
}

impl BitReader {
	pub fn new(bits: Vec<u8>) -> Self{
		return BitReader { bits, pos: 0 };
	}

	pub fn pos(&self) -> usize {
		return self.pos;
	}

	pub fn next_n(&mut self, n: usize) -> &[u8] {
		let d = &self.bits[self.pos..self.pos+n];
		self.pos += n;
		return d;
	}
}

#[derive(Debug)]
pub enum Packet {
	LiteralPacket(LiteralData),
	OperatorPacket(OperatorData)
}

impl Packet {
	pub fn evaluate(&self) -> u64 {
		return match self {
			Packet::LiteralPacket(p) => p.literal,
			Packet::OperatorPacket(p) => p.evaluate(),
		}
	}

	pub fn version(&self) -> u64 {
		return match self {
			Packet::LiteralPacket(p) => p.header.version,
			Packet::OperatorPacket(p) => p.header.version,
		}
	}
}

#[derive(Debug)]
pub struct HeaderData {
	version: u64,
	type_id: u64
}

#[derive(Debug)]
pub struct LiteralData {
	header: HeaderData,
	literal: u64
}

impl LiteralData {
	pub fn new(version: u64, literal: u64) -> Self {
		let header = HeaderData { version, type_id :4 };
		return LiteralData { header, literal };
	}
}

#[derive(Debug)]
pub struct OperatorData {
	header: HeaderData,
	packets: Vec<Packet>
}

impl OperatorData {
	pub fn new(version: u64, type_id: u64) -> Self {
		let header = HeaderData { version, type_id };
		return OperatorData { header, packets: Vec::new() };
	}

	pub fn add_packet(&mut self, packet: Packet) {
		self.packets.push(packet);
	}

	pub fn packets(&self) -> impl Iterator<Item=&Packet> {
		return self.packets.iter();
	}

	pub fn evaluate(&self) -> u64 {
		return match self.header.type_id {
			0 => self.packets.iter().map(|p| p.evaluate()).sum(),
			1 => self.packets.iter().map(|p| p.evaluate()).product(),
			2 => self.packets.iter().map(|p| p.evaluate()).min().unwrap(),
			3 => self.packets.iter().map(|p| p.evaluate()).max().unwrap(),
			5 => if self.packets[0].evaluate() > self.packets[1].evaluate() { 1 } else { 0 },
			6 => if self.packets[0].evaluate() < self.packets[1].evaluate() { 1 } else { 0 },
			7 => if self.packets[0].evaluate() == self.packets[1].evaluate() { 1 } else { 0 },
			_ => panic!("Unsupported type id!")
		};
	}
}
