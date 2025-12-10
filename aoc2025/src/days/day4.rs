use aoc::{DayResult, DaySolver};

fn check(grid: &mut Vec<Vec<u8>>, remove: bool) -> usize {
	let mut papers = 0;

	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			if grid[y][x] != b'@' {
				continue;
			}

			let mut neighbors = 0;

			for gy in y.saturating_sub(1)..=(grid.len() - 1).min(y + 1) {
				for gx in x.saturating_sub(1)..=(grid[y].len() - 1).min(x + 1) {
					if gx == x && gy == y {
						continue;
					}

					if grid[gy][gx] == b'@' {
						neighbors += 1;
					}
				}
			}

			if neighbors < 4 {
				if remove {
					grid[y][x] = b'.';
				}

				papers += 1;
			}
		}
	}

	papers
}

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let mut grid: Vec<_> = input
			.lines()
			.map(|l| l.as_bytes())
			.map(|l| l.to_vec())
			.collect();

		DayResult::success(check(&mut grid, false))
	}

	fn two(&self, input: &str) -> DayResult {
		let mut grid: Vec<_> = input
			.lines()
			.map(|l| l.as_bytes())
			.map(|l| l.to_vec())
			.collect();

		let mut total = 0;

		loop {
			let removed = check(&mut grid, true);
			total += removed;
			if removed == 0 {
				break;
			}
		}

		DayResult::success(total)
	}
}
