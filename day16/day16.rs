mod packet;
use crate::packet::{BitReader, LiteralData, OperatorData, Packet};

fn get_bits(byte: u8) -> impl Iterator<Item=u8> {
	return (0..4).rev()
		.map(move |i| if (byte & (1 << i)) == 0 { 0 } else { 1 });
}

fn bits_to_int(bits: &[u8]) -> u64 {
	return bits.iter().rev()
		.enumerate()
		.fold(0, |acc, (i, v)|
			if v == &1 { acc + 2_u64.pow(i as u32)} else { acc });
}

fn parse_packet(reader: &mut BitReader) -> Packet {
	let version = bits_to_int(reader.next_n(3));
	let type_id = bits_to_int(reader.next_n(3));

	if type_id == 4 {
		let mut value = Vec::new();
		let mut status = 1;
		while status == 1 {
			status = reader.next_n(1)[0];
			reader.next_n(4).iter().for_each(|&bit| value.push(bit));
		}
		if value.len() % 4 != 0 {
			reader.next_n(4 - (value.len() % 4));
		}

		return Packet::LiteralPacket(LiteralData::new(version, bits_to_int(&value)));
	}

	let mut packet = OperatorData::new(version, type_id);

	let length_type_id = reader.next_n(1)[0];
	if length_type_id == 0 {
		let end = bits_to_int(reader.next_n(15)) as usize + reader.pos();

		while reader.pos() < end {
			packet.add_packet(parse_packet(reader));
		}
	} else {
		let num_packets = bits_to_int(reader.next_n(11));

		for _ in 0..num_packets {
			packet.add_packet(parse_packet(reader));
		}
	}

	return Packet::OperatorPacket(packet);
}

fn version_sum(packet: &Packet) -> u64 {
	return packet.version() + match packet {
		Packet::OperatorPacket(p) =>
			p.packets().map(|sp| version_sum(sp)).sum::<u64>(),
		_ => 0,
	};
}

fn part1(bits: Vec<u8>) -> u64 {
	let mut reader = BitReader::new(bits);
	let packet = parse_packet(&mut reader);

	return version_sum(&packet);
}

fn part2(bits: Vec<u8>) -> u64 {
	let mut reader = BitReader::new(bits);
	let packet = parse_packet(&mut reader);

	if let Packet::OperatorPacket(d) = packet {
		return d.evaluate();
	}
	panic!("Expected operator packet as root!");
}

fn main() {
	let content = std::fs::read("inputs/day16_input.txt")
		.expect("Could not read file!");
	let bits: Vec<u8> = content.iter()
		.filter(|&&byte| (byte >= 48 && byte < 58) || (byte >= 65 && byte <= 90))
		.map(|&byte| if byte >= 58 { byte - 55} else { byte - 48 } )
		.flat_map(|c| get_bits(c))
		.collect();

	println!("Part1: {}", part1(bits.clone()));
	println!("Part2: {}", part2(bits));
}