use std::collections::{HashMap, HashSet};

use aoc::{DayResult, DaySolver};

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let mut lines = input.lines().step_by(2);
		let (start, width) = {
			let line = lines.next().unwrap();
			let start = line.find('S').unwrap();
			let width = line.len();
			(start, width)
		};

		let mut splits = 0;
		let mut beams = HashSet::from([start]);
		beams.reserve(width);
		let mut new_beams = HashSet::with_capacity(width);

		for line in lines {
			let splitters = line
				.as_bytes()
				.iter()
				.enumerate()
				.filter(|&(_, b)| *b == b'^')
				.map(|(i, _)| i);

			for splitter in splitters {
				if beams.contains(&splitter) {
					splits += 1;
					new_beams.insert(splitter - 1);
					new_beams.insert(splitter + 1);
					beams.remove(&splitter);
				}
			}

			beams.extend(&new_beams);
			new_beams.clear();
		}

		DayResult::success(splits)
	}

	fn two(&self, input: &str) -> DayResult {
		let mut lines = input.lines().step_by(2);
		let (start, width) = {
			let line = lines.next().unwrap();
			let start = line.find('S').unwrap();
			let width = line.len();
			(start, width)
		};

		let mut beams = HashMap::from([(start, 1usize)]);
		beams.reserve(width);
		let mut new_beams = HashMap::with_capacity(width);

		for line in lines {
			let splitters = line
				.as_bytes()
				.iter()
				.enumerate()
				.filter(|&(_, b)| *b == b'^')
				.map(|(i, _)| i);

			for splitter in splitters {
				if beams.contains_key(&splitter) {
					let value = *beams.get(&splitter).unwrap();
					new_beams
						.entry(splitter - 1)
						.and_modify(|v| *v += value)
						.or_insert(value + beams.get(&(splitter - 1)).unwrap_or(&0));
					new_beams
						.entry(splitter + 1)
						.and_modify(|v| *v += value)
						.or_insert(value + beams.get(&(splitter + 1)).unwrap_or(&0));
					beams.remove(&splitter);
				}
			}

			beams.extend(&new_beams);
			new_beams.clear();
		}

		DayResult::success(beams.values().fold(0, |a, b| a + *b))
	}
}
