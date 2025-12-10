use aoc::{DayResult, DaySolver};

fn parse_button(line: &str) -> (i64, i64) {
	let line = &line[12..];

	let (x, y) = line.split_once(", Y+").unwrap();

	(x.parse().unwrap(), y.parse().unwrap())
}

fn parse_prize(line: &str) -> (i64, i64) {
	let line = &line[9..];

	let (x, y) = line.split_once(", Y=").unwrap();

	(x.parse().unwrap(), y.parse().unwrap())
}

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let mut lines = input.lines().peekable();

		let mut tokens = 0;

		while lines.peek().is_some() {
			let (ax, ay) = parse_button(lines.next().unwrap());
			let (bx, by) = parse_button(lines.next().unwrap());
			let (x, y) = parse_prize(lines.next().unwrap());
			lines.next();

			// ax * a + bx * b = x
			// ay * a + by * b = y

			let d = (ax * by - ay * bx) as f64;

			let da = x * by - y * bx;
			let db = y * ax - x * ay;

			let a = da as f64 / d;
			let b = db as f64 / d;

			if a.fract() != 0. || b.fract() != 0. {
				continue;
			}

			if a > 100. || b > 100. {
				continue;
			}

			tokens += (a as i64) * 3 + b as i64
		}

		DayResult::success(tokens)
	}

	fn two(&self, input: &str) -> DayResult {
		let mut lines = input.lines().peekable();

		let mut tokens = 0;

		while lines.peek().is_some() {
			let (ax, ay) = parse_button(lines.next().unwrap());
			let (bx, by) = parse_button(lines.next().unwrap());
			let (mut x, mut y) = parse_prize(lines.next().unwrap());
			lines.next();

			x += 10000000000000;
			y += 10000000000000;

			// ax * a + bx * b = x
			// ay * a + by * b = y

			let d = (ax * by - ay * bx) as f64;

			let da = x * by - y * bx;
			let db = y * ax - x * ay;

			let a = da as f64 / d;
			let b = db as f64 / d;

			if a.fract() != 0. || b.fract() != 0. {
				continue;
			}

			tokens += (a as i64) * 3 + b as i64
		}

		DayResult::success(tokens)
	}
}
