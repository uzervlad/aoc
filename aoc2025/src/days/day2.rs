use aoc::{DayResult, DaySolver};

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		fn id_invalid(id: u64) -> bool {
			match id {
				11..=99 => id % 11 == 0,
				1010..=9999 => id % 101 == 0,
				100100..=999999 => id % 1001 == 0,
				10001000..=99999999 => id % 10001 == 0,
				1000010000..=9999999999 => id % 100001 == 0,
				_ => false,
			}
		}

		let mut sum = 0;
		let ranges = input.split(',');
		for range in ranges {
			let (from, to) = range.split_once('-').unwrap();
			let from = from.parse::<u64>().unwrap();
			let to = to.parse::<u64>().unwrap();

			sum += (from..=to).filter(|&id| id_invalid(id)).sum::<u64>();
		}
		DayResult::success(sum)
	}

	fn two(&self, input: &str) -> DayResult {
		fn id_invalid(id: u64) -> bool {
			match id {
				10..=99 => id % 11 == 0,
				111..=999 => id % 111 == 0,
				1010..=9999 => id % 101 == 0,
				11111..=99999 => id % 11111 == 0,
				100100..=999999 => (id % 1001 == 0) || (id % 10101 == 0),
				1111111..=9999999 => id % 1111111 == 0,
				10001000..=99999999 => (id % 1010101 == 0) || (id % 10001 == 0),
				111111111..=999999999 => id % 1001001 == 0,
				1000010000..=9999999999 => (id % 101010101 == 0) || (id % 100001 == 0),
				_ => false,
			}
		}

		let mut sum = 0;
		let ranges = input.split(',');
		for range in ranges {
			let (from, to) = range.split_once('-').unwrap();
			let from = from.parse::<u64>().unwrap();
			let to = to.parse::<u64>().unwrap();

			sum += (from..=to)
				.filter_map(|id| if id_invalid(id) { Some(id) } else { None })
				.sum::<u64>();
		}

		DayResult::success(sum)
	}
}
