use std::ops::RangeInclusive;

use aoc::{DayResult, DaySolver};

fn intersect<I: PartialOrd>(a: &RangeInclusive<I>, b: &RangeInclusive<I>) -> bool {
	a.start() <= b.end() && b.start() <= a.end()
}

fn join<I: Ord + Copy>(a: &RangeInclusive<I>, b: &RangeInclusive<I>) -> RangeInclusive<I> {
	(*a.start()).min(*b.start())..=(*a.end()).max(*b.end())
}

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let (ranges, ingredients) = input.trim().split_once("\n\n").unwrap();

		let ranges: Vec<RangeInclusive<u64>> = ranges
			.lines()
			.map(|line| {
				let (from, to) = line.split_once('-').unwrap();
				from.parse().unwrap()..=to.parse().unwrap()
			})
			.collect();

		let ingredients = ingredients
			.lines()
			.map(|line| line.parse().unwrap())
			.filter(|ingredient| ranges.iter().any(|range| range.contains(&ingredient)))
			.count();

		DayResult::success(ingredients)
	}

	fn two(&self, input: &str) -> DayResult {
		let (input, _) = input.trim().split_once("\n\n").unwrap();

		let mut ranges: Vec<RangeInclusive<u64>> = input
			.lines()
			.map(|line| {
				let (from, to) = line.split_once('-').unwrap();
				from.parse().unwrap()..=to.parse().unwrap()
			})
			.collect();
		ranges.sort_unstable_by_key(|r| *r.start());

		let mut flat_ranges = vec![];

		for range in ranges {
			if let Some(flat) = flat_ranges.last_mut() {
				if intersect(flat, &range) {
					let r = join(flat, &range);
					*flat = r;
					continue;
				}
			}

			flat_ranges.push(range);
		}

		let ingredients = flat_ranges
			.iter()
			.map(|range| range.end() - range.start() + 1)
			.sum::<u64>();

		DayResult::success(ingredients)
	}
}
