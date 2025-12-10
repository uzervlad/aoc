use std::cmp::Ordering;

use aoc::{DayResult, DaySolver};
use itertools::Itertools;

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let mut safe = 0;

		'a: for line in input.lines() {
			let levels = line
				.split_ascii_whitespace()
				.map(|level| unsafe { level.parse::<u8>().unwrap_unchecked() });

			let mut ordering = None;

			for (a, b) in levels.tuple_windows() {
				ordering = match (ordering, b.cmp(&a)) {
					(None, ord) => Some(ord),
					(Some(ord), new_ord) if ord != new_ord => {
						continue 'a;
					}
					_ => ordering,
				};

				match b.abs_diff(a) {
					1 | 2 | 3 => {}
					_ => continue 'a,
				}
			}

			safe += 1;
		}

		DayResult::success(safe)
	}

	fn two(&self, input: &str) -> DayResult {
		let mut safe = 0;

		'a: for line in input.lines() {
			let levels = [0]
				.iter()
				.map(|_| 0)
				.chain(
					line.split_ascii_whitespace()
						.map(|level| level.parse::<u8>().unwrap()),
				)
				.chain([0].iter().map(|_| 0));

			let mut ordering = None;
			let mut has_skipped = false;
			let mut skipped = false;

			for (a, b, c) in levels.tuple_windows() {
				if a == 0 {
					match c.cmp(&b) {
						Ordering::Equal => {
							has_skipped = true;
							skipped = true;
							continue;
						}
						ord => {
							ordering = Some(ord);
							continue;
						}
					}
				}

				if skipped {
					skipped = false;
					continue;
				}

				let mut safe = true;

				ordering = match (ordering, b.cmp(&a)) {
					(_, Ordering::Equal) => {
						safe = false;

						None
					}
					(None, ord) => Some(ord),
					(Some(ord), new_ord) => {
						if new_ord != ord {
							safe = false;
						}

						Some(ord)
					}
				};

				match b.abs_diff(a) {
					1..=3 => {}
					_ => safe = false,
				}

				if !safe && has_skipped {
					continue 'a;
				}

				if !safe && !has_skipped && c != 0 {
					has_skipped = true;
					skipped = true;

					ordering = match (ordering, c.cmp(&a)) {
						(_, Ordering::Equal) => continue 'a,
						(None, ord) => Some(ord),
						(Some(ord), new_ord) => {
							if new_ord != ord {
								continue 'a;
							}

							Some(ord)
						}
					};

					match c.abs_diff(a) {
						1..=3 => {}
						_ => continue 'a,
					}
				}
			}

			safe += 1;
		}

		DayResult::success(safe)
	}
}
