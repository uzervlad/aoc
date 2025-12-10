use aoc::{DayResult, DaySolver};

const DIRECTIONS: [(isize, isize); 8] = [
	(1, 0),
	(-1, 0),
	(0, 1),
	(0, -1),
	(1, 1),
	(-1, -1),
	(1, -1),
	(-1, 1),
];

#[derive(PartialEq)]
enum Letter {
	X,
	M,
	A,
	S,
}

struct Grid {
	size: usize,
	grid: Vec<Letter>,
}

impl Grid {
	fn push_letter(&mut self, letter: &u8) {
		self.grid.push(match letter {
			b'X' => Letter::X,
			b'M' => Letter::M,
			b'A' => Letter::A,
			b'S' => Letter::S,
			_ => unreachable!(),
		})
	}
}

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let mut rows = input.lines();

		let Some(first) = rows.next() else {
			unreachable!()
		};

		let width = first.len();

		let mut x_s = Vec::with_capacity(width * width / 2);

		let mut grid = Grid {
			size: width,
			grid: Vec::with_capacity(width * width),
		};

		for char in first.as_bytes() {
			if matches!(char, b'X') {
				x_s.push((grid.grid.len() % grid.size, grid.grid.len() / grid.size));
			}

			grid.push_letter(char);
		}

		for line in rows {
			for char in line.as_bytes() {
				if matches!(char, b'X') {
					x_s.push((grid.grid.len() % grid.size, grid.grid.len() / grid.size));
				}

				grid.push_letter(char);
			}
		}

		let xmas = x_s
			.iter()
			.map(|&(x, y)| {
				DIRECTIONS
					.iter()
					.map(|&direction| {
						for (i, c) in [Letter::M, Letter::A, Letter::S].iter().enumerate() {
							let x = x as isize + direction.0 * (i as isize + 1);
							let y = y as isize + direction.1 * (i as isize + 1);

							if x == -1
								|| x == grid.size as isize
								|| y == -1 || y == grid.size as isize
							{
								return 0;
							}

							if grid.grid[(y * grid.size as isize + x) as usize] != *c {
								return 0;
							}
						}
						1
					})
					.sum::<usize>()
			})
			.sum::<usize>();

		DayResult::success(xmas)
	}

	fn two(&self, input: &str) -> DayResult {
		let mut rows = input.lines();

		let Some(first) = rows.next() else {
			unreachable!()
		};

		let width = first.len();

		let mut a_s = Vec::with_capacity(width * width / 2);

		let mut grid = Grid {
			size: width,
			grid: Vec::with_capacity(width * width),
		};

		for char in first.as_bytes() {
			grid.push_letter(char);
		}

		for line in rows {
			for char in line.as_bytes() {
				if matches!(char, b'A')
					&& !(grid.grid.len() % grid.size == 0
						|| grid.grid.len() % grid.size == grid.size - 1
						|| grid.grid.len() / grid.size == grid.size - 1)
				{
					a_s.push(grid.grid.len() as isize);
				}

				grid.push_letter(char);
			}
		}

		let size = grid.size as isize;

		let x_mas = a_s
			.iter()
			.map(|&a| {
				[(size + 1, -size - 1), (size - 1, -size + 1)]
					.iter()
					.all(|&direction| match grid.grid[(a + direction.0) as usize] {
						Letter::M => grid.grid[(a + direction.1) as usize] == Letter::S,
						Letter::S => grid.grid[(a + direction.1) as usize] == Letter::M,
						_ => false,
					})
			})
			.filter(|&x| x)
			.count();

		DayResult::success(x_mas)
	}
}
