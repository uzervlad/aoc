use aoc::{DayResult, DaySolver};

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let mut sum = 0;

		for line in input.lines() {
			let mut joltage = [0_u8, 0];
			let len = line.len();
			for (idx, ch) in line.chars().enumerate() {
				let j = ch as u8 - b'0';
				if idx == len - 1 {
					joltage[1] = joltage[1].max(j);
				} else {
					if joltage[0] < j {
						joltage = [j, 0];
					} else {
						joltage[1] = joltage[1].max(j);
					}
				}
			}

			sum += (joltage[0] * 10 + joltage[1]) as i64;
		}

		DayResult::success(sum)
	}

	fn two(&self, input: &str) -> DayResult {
		let mut sum = 0;

		for line in input.lines() {
			let mut joltage = [0_u8; 12];
			let len = line.len();
			for (idx, ch) in line.chars().enumerate() {
				let mi = 12 - (len - idx).min(12);
				let j = ch as u8 - b'0';
				let bi = joltage
					.iter()
					.enumerate()
					.find(|&(idx, &b)| idx >= mi && j > b)
					.map(|(idx, _)| idx);
				if let Some(bi) = bi {
					joltage[bi] = j;
					joltage[(bi + 1)..].iter_mut().for_each(|x| *x = 0);
				}
			}

			sum += joltage
				.iter()
				.enumerate()
				.map(|(idx, &b)| (b as i64) * 10_i64.pow(11 - idx as u32))
				.sum::<i64>();
		}

		DayResult::success(sum)
	}
}
