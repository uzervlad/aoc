use aoc::{DayResult, DaySolver};

fn parse_present<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> usize {
	lines.next();

	let mut p = 0;

	for _ in 0..3 {
		let line = lines.next().unwrap();
		for i in 0..3 {
			if line.as_bytes()[i] == b'#' {
				p += 1;
			}
		}
	}

	lines.next();

	p
}

#[derive(Debug)]
struct Region {
	size: [usize; 2],
	presents: [usize; 6],
}

fn parse_region(line: &str) -> Region {
	let (size, presents) = line.split_once(':').unwrap();
	let (w, h) = size.split_once('x').unwrap();
	let size = [w.parse().unwrap(), h.parse().unwrap()];
	let mut presents = presents
		.trim()
		.split_whitespace()
		.filter_map(|p| p.parse().ok());

	Region {
		size,
		presents: [
			presents.next().unwrap(),
			presents.next().unwrap(),
			presents.next().unwrap(),
			presents.next().unwrap(),
			presents.next().unwrap(),
			presents.next().unwrap(),
		],
	}
}

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let mut lines = input.lines();

		let presents: [usize; _] = [
			parse_present(&mut lines),
			parse_present(&mut lines),
			parse_present(&mut lines),
			parse_present(&mut lines),
			parse_present(&mut lines),
			parse_present(&mut lines),
		];

		let mut total = 0;

		for line in lines.filter(|l| !l.is_empty()) {
			let region = parse_region(line);

			let total_area = region.size[0] * region.size[1];
			let presents_area = presents
				.iter()
				.enumerate()
				.map(|(i, p)| region.presents[i] * p)
				.sum::<usize>();

			if total_area >= presents_area {
				total += 1;
			}
		}

		DayResult::success(total)
	}

	fn two(&self, _: &str) -> DayResult {
		DayResult::Note("Final day".into())
	}
}
