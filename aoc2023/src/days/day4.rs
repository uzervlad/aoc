use std::collections::HashSet;

use aoc::{DayResult, DaySolver};

fn get_score(line: &str) -> u32 {
  let (_, numbers) = line.split_once(": ").unwrap();
  let (win, scratch) = numbers.split_once(" | ").unwrap();
  let win: HashSet<u32> = HashSet::from_iter(win.split(" ").filter(|s| !s.is_empty()).flat_map(str::parse::<u32>));
  let scratch: HashSet<u32> = HashSet::from_iter(scratch.split(" ").filter(|s| !s.is_empty()).flat_map(str::parse::<u32>));
  win.intersection(&scratch).count() as u32
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    DayResult::Success(
      input
        .lines()
        .map(get_score)
        .map(|score| if score > 0 { 1 << score - 1 } else { 0 })
        .sum::<u32>() as i64)
  }

  fn two(&self, input: &str) -> DayResult {
    let scores: Vec<_> = input.lines().map(get_score).collect();

    let mut cards = vec![1; scores.len()];

    for i in (0..cards.len()).rev() {
      for j in 1..=scores[i] as usize {
        cards[i] += cards[i + j];
      }
    }

    DayResult::Success(cards.iter().sum::<u32>() as i64)
  }
}