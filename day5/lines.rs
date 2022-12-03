pub type Point = (i32, i32);

fn sig(x: i32) -> i32 {
	if x == 0 {
		return 0;
	}
	return x / x.abs();
}

pub fn parse_point(s: &str) -> Option<Point> {
	let mut point = s.split(",")
		.map(|c| c.parse::<i32>().unwrap());

	return Some((point.next()?, point.next()?));
}

pub fn parse_line(l: &str) -> Option<(Point, Point)> {
	let mut p = l.split(" -> ");

	return Some((parse_point(p.next()?)?, parse_point(p.next()?)?))
}

pub fn points_on_line(start: Point, end: Point) -> Vec<Point> {
	let dx = sig(end.0 - start.0);
	let dy = sig(end.1 - start.1);

	let mut points = Vec::new();

	let mut p = start;
	while p != end {
		points.push(p);
		p.0 += dx;
		p.1 += dy;
	}
	points.push(p);

	return points;
}

#[cfg(test)]
mod tests {
	use super::{sig, parse_point, parse_line, points_on_line};

	#[test]
	fn test_sig1() {
		let res = sig(0);
		assert_eq!(res, 0);
	}

	#[test]
	fn test_sig2() {
		let res = sig(10);
		assert_eq!(res, 1);
	}

	#[test]
	fn test_sig3() {
		let res = sig(-10);
		assert_eq!(res, -1);
	}

	#[test]
	fn test_parse_point() {
		let res = parse_point("10,14").unwrap();
		assert_eq!(res, (10, 14))
	}

	#[test]
	fn test_parse_line() {
		let res = parse_line("10,12 -> 10,14").unwrap();
		assert_eq!(res, ((10, 12), (10, 14)))
	}

	#[test]
	fn test_points_on_line1() {
		let res = points_on_line((0, 0), (2, 2));
		assert_eq!(res, vec![(0, 0), (1, 1), (2, 2)])
	}

	#[test]
	fn test_points_on_line2() {
		let res = points_on_line((5, 5), (3, 3));
		assert_eq!(res, vec![(5, 5), (4, 4), (3, 3)])
	}
}
