use std::collections::{VecDeque, HashSet};

pub type Point = (i32, i32, i32);

pub fn rotate((x, y, z): Point, i: usize) -> Point {
	return match i {
		0 => (x, y, z),
		1 => (x, z, -y),
		2 => (x, -y, -z),
		3 => (x, -z, y),
		4 => (y, x, -z),
		5 => (y, z, x),
		6 => (y, -x, z),
		7 => (y, -z, -x),
		8 => (z, x, y),
		9 => (z, y, -x),
		10 => (z, -x, -y),
		11 => (z, -y, x),
		12 => (-x, y, -z),
		13 => (-x, z, y),
		14 => (-x, -y, z),
		15 => (-x, -z, -y),
		16 => (-y, x, z),
		17 => (-y, z, -x),
		18 => (-y, -x, -z),
		19 => (-y, -z, x),
		20 => (-z, x, -y),
		21 => (-z, y, x),
		22 => (-z, -x, y),
		23 => (-z, -y, -x),
		_ => panic!("Not supported!")
	}
}
fn add(p1: &Point, p2: &Point) -> Point {
	return (p1.0 + p2.0, p1.1 + p2.1, p1.2 + p2.2);
}

fn sub(p1: &Point, p2: &Point) -> Point {
	return (p1.0 - p2.0, p1.1 - p2.1, p1.2 - p2.2);
}

pub fn distance(p1: &Point, p2: &Point) -> u32 {
	return ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()) as u32;
}

pub fn reconstruct_scanner_map(mut scanners: Vec<Vec<Point>>) -> (Vec<Point>, HashSet<Point>) {
	let mut known_beacons: HashSet<Point> = scanners.remove(0).into_iter().collect();
	let mut known_scanners = Vec::new();
	let mut unknown: VecDeque<Vec<Vec<Point>>> = scanners.into_iter()
		.map(|points| (0..24).map(|i| points.iter()
			.map(|p| rotate(*p, i)).collect())
			.collect()
		).collect();

	'scanner: while !unknown.is_empty() {
		let scanner = unknown.pop_front();

		for rotation in scanner.iter() {
			for points in rotation {
				for i in known_beacons.iter() {
					for j in points.iter() {
						let translation = sub(i, j);

						let matches = points.iter()
							.filter(|b| known_beacons.contains(&add(b, &translation)))
							.count();

						if matches >= 12 {
							known_beacons.extend(points.iter().map(|p| add(p, &translation)));
							known_scanners.push(translation);
							continue 'scanner;
						}
					}
				}
			}
		}
		unknown.push_back(scanner.unwrap());
	}

	return (known_scanners, known_beacons);
}
