use aoc::{DayResult, DaySolver};

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		fn parse_lights(diagram: &str) -> u16 {
			diagram
				.as_bytes()
				.iter()
				.skip(1)
				.take(diagram.len() - 2)
				.enumerate()
				.map(|(i, &a)| if a == b'#' { 1 << i } else { 0 })
				.sum()
		}

		fn parse_wiring(wiring: &str) -> u16 {
			wiring
				.trim_start_matches('(')
				.trim_end_matches(')')
				.split(',')
				.map(str::parse::<u16>)
				.map(Result::unwrap)
				.map(|l| 1 << l)
				.sum()
		}

		let mut total = 0;

		for line in input.lines() {
			let mut parts = line.split_whitespace();

			let diagram = parse_lights(parts.next().unwrap());
			let wiring: Vec<_> = parts
				.filter(|p| p.starts_with('('))
				.map(parse_wiring)
				.collect();

			let mut least_presses = usize::MAX;

			for n in 1_usize..(2 << wiring.len()) {
				let mut result = 0;

				for i in 0..wiring.len() {
					if n & (1 << i) != 0 {
						result ^= wiring[i];
					}
				}

				if result == diagram {
					least_presses = least_presses.min(n.count_ones() as _)
				}
			}

			total += least_presses;
		}

		DayResult::success(total)
	}

	fn two(&self, _: &str) -> DayResult {
		DayResult::Todo
	}
}
