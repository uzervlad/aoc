use std::iter;

use aoc::{DayResult, DaySolver};
pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let mut blocks: Vec<Option<u16>> = Vec::with_capacity(20000 * 9);

		let mut uid = 0;
		let mut free = false;

		for &map in input.as_bytes() {
			let size = map - b'0';
			if free {
				blocks.extend(iter::repeat(None).take(size as usize));
			} else {
				blocks.extend(iter::repeat(Some(uid)).take(size as usize));
				uid += 1;
			}
			free = !free;
		}

		let mut sum = 0;

		let mut end = blocks.len();

		for i in 0..blocks.len() {
			if i >= end {
				break;
			}

			if blocks[i].is_none() {
				for j in (i + 1..end).rev() {
					if let Some(jj) = blocks[j] {
						end = j;
						sum += i * jj as usize;
						break;
					}
				}
			} else {
				sum += i * blocks[i].unwrap() as usize;
			}
		}

		DayResult::success(sum)
	}

	fn two(&self, _: &str) -> DayResult {
		DayResult::Todo
	}
}
