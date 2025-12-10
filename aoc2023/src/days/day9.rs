use aoc::{DayResult, DaySolver};

fn extrapolate_sequence(sequence: &Vec<i32>, backwards: bool) -> i32 {
	let next_sequence: Vec<i32> = (1..sequence.len())
		.map(|i| sequence[i] - sequence[i - 1])
		.collect();

	if next_sequence.iter().all(|n| *n == 0) {
		sequence[0]
	} else if backwards {
		sequence[0] - extrapolate_sequence(&next_sequence, backwards)
	} else {
		sequence[sequence.len() - 1] + extrapolate_sequence(&next_sequence, backwards)
	}
}

fn solve(sequences: &Vec<Vec<i32>>, backwards: bool) -> i32 {
	sequences
		.iter()
		.map(|sequence| extrapolate_sequence(sequence, backwards))
		.sum::<i32>()
}

fn get_sequences(input: &str) -> Vec<Vec<i32>> {
	input
		.lines()
		.map(|line| {
			line.split_whitespace()
				.flat_map(|n| n.parse::<i32>())
				.collect()
		})
		.collect()
}

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		DayResult::success(solve(&get_sequences(input), false))
	}

	fn two(&self, input: &str) -> DayResult {
		DayResult::success(solve(&get_sequences(input), true))
	}
}
