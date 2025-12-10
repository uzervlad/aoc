use aoc::{DayResult, DaySolver};
use rayon::{iter::ParallelIterator, str::ParallelString};

trait Stone {
	fn digits(&self) -> u32;
	fn blinks(&self, blinks: u8) -> u64;
}

impl Stone for u64 {
	fn digits(&self) -> u32 {
		let mut digits = 0;
		let mut value = *self;

		while value > 0 {
			digits += 1;
			value /= 10;
		}

		digits
	}

	fn blinks(&self, mut blinks: u8) -> u64 {
		if blinks == 0 {
			1
		} else {
			blinks -= 1;
			match *self {
				0 => 1.blinks(blinks),
				v => {
					let d = v.digits();
					if d % 2 == 0 {
						let shift = 10_u64.pow(d / 2);
						(v / shift).blinks(blinks) + (v % shift).blinks(blinks)
					} else {
						(v * 2024).blinks(blinks)
					}
				}
			}
		}
	}
}

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let stones = input
			.par_split_ascii_whitespace()
			.filter_map(|s| s.parse().ok())
			.map(|s: u64| s.blinks(50))
			.sum::<u64>();

		DayResult::success(stones)
	}

	fn two(&self, _: &str) -> DayResult {
		DayResult::Todo
	}
}
