use std::collections::HashSet;

use aoc::{DayResult, DaySolver};

type Position = (isize, isize);

trait DirectionTrait {
	fn turn(&mut self);
	fn step(&self, rhs: Position) -> Position;
	fn out_of_bounds(&self, size: isize) -> bool;
}

impl DirectionTrait for Position {
	fn turn(&mut self) {
		*self = match self {
			(0, -1) => (1, 0),
			(1, 0) => (0, 1),
			(0, 1) => (-1, 0),
			(-1, 0) => (0, -1),
			_ => unreachable!(),
		}
	}

	fn step(&self, rhs: Position) -> Position {
		(self.0 + rhs.0, self.1 + rhs.1)
	}

	fn out_of_bounds(&self, size: isize) -> bool {
		self.0 < 0 || self.0 >= size || self.1 < 0 || self.1 >= size
	}
}

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let mut walls = HashSet::<Position>::new();
		let mut visited = HashSet::<Position>::new();

		let mut current_position: Position = (0, 0);
		let mut direction: Position = (0, -1);

		let mut size = 0;

		let mut x = 0;
		let mut y = 0;

		for &b in input.as_bytes() {
			match b {
				b'#' => {
					walls.insert((x, y));
					x += 1;
				}
				b'^' => {
					current_position = (x, y);
					visited.insert(current_position);
					x += 1;
				}
				b'\n' => {
					size = x.max(size);
					y += 1;
					x = 0;
				}
				b'\r' => {}
				_ => {
					x += 1;
				}
			}
		}

		loop {
			let next_position = current_position.step(direction);
			if next_position.out_of_bounds(size) {
				break;
			}
			if walls.contains(&next_position) {
				direction.turn();
			} else {
				current_position = next_position;
				visited.insert(current_position);
			}
		}

		DayResult::success(visited.len())
	}

	fn two(&self, input: &str) -> DayResult {
		let mut walls = HashSet::<Position>::new();
		let mut visited = HashSet::<Position>::new();
		let mut visited_with_direction = HashSet::<(Position, Position)>::new();

		let mut starting_position = (0, 0);
		let mut current_position: Position = (0, 0);
		let mut direction: Position = (0, -1);

		let mut size = 0;

		let mut x = 0;
		let mut y = 0;

		for &b in input.as_bytes() {
			match b {
				b'#' => {
					walls.insert((x, y));
					x += 1;
				}
				b'^' => {
					starting_position = (x, y);
					current_position = (x, y);
					visited.insert(current_position);
					x += 1;
				}
				b'\n' => {
					size = x.max(size);
					y += 1;
					x = 0;
				}
				b'\r' => {}
				_ => {
					x += 1;
				}
			}
		}

		loop {
			let next_position = current_position.step(direction);
			if next_position.out_of_bounds(size) {
				break;
			}

			if walls.contains(&next_position) {
				direction.turn();
				visited_with_direction.insert((current_position, direction));
			} else {
				current_position = next_position;
				visited.insert(current_position);
				visited_with_direction.insert((current_position, direction));
			}
		}

		let mut possible_walls = HashSet::new();

		'a: for (position, direction) in &visited_with_direction {
			let potential_wall = position.step(*direction);

			if potential_wall == starting_position || potential_wall.out_of_bounds(size) {
				continue;
			}

			let mut position = starting_position;
			let mut direction = (0, -1);

			let mut loop_visited = HashSet::new();

			loop {
				let next_position = position.step(direction);
				if next_position.out_of_bounds(size) {
					continue 'a;
				}

				if walls.contains(&next_position) || potential_wall == next_position {
					direction.turn();
				} else {
					position = next_position;
				}

				if !loop_visited.insert((position, direction)) {
					break;
				}
			}

			possible_walls.insert(potential_wall);
		}

		DayResult::success(possible_walls.len())
	}
}
