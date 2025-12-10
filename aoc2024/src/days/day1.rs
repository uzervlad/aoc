use std::iter::zip;

use aoc::{DayResult, DaySolver};

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let mut a = [0; 1000];
		let mut b = [0; 1000];

		for (i, (_a, _b)) in input
			.lines()
			.map(|line| unsafe {
				(
					u32::from_str_radix(&line[0..5], 10).unwrap_unchecked(),
					u32::from_str_radix(&line[8..13], 10).unwrap_unchecked(),
				)
			})
			.enumerate()
		{
			a[i] = _a;
			b[i] = _b;
		}

		a.sort_unstable();
		b.sort_unstable();

		let sum = zip(a.iter(), b.iter()).fold(0, |acc, (a, b)| acc + a.abs_diff(*b));
		DayResult::success(sum)
	}

	fn two(&self, input: &str) -> DayResult {
		let mut a = [0; 1000];
		let mut b = [0; 100000];

		for (i, (_a, _b)) in input
			.lines()
			.map(|line| unsafe {
				(
					u32::from_str_radix(&line[0..5], 10).unwrap_unchecked(),
					u32::from_str_radix(&line[8..13], 10).unwrap_unchecked(),
				)
			})
			.enumerate()
		{
			a[i] = _a;
			b[_b as usize] += 1;
		}

		let sum = a.iter().fold(0, |acc, a| acc + a * b[*a as usize]);

		DayResult::success(sum)
	}
}
