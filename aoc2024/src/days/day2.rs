use aoc::{DayResult, DaySolver};
use itertools::Itertools;

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut safe = 0;

    'a: for line in input.lines() {
      let levels = line.split_ascii_whitespace()
        .map(|level| level.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

      let mut increasing = None;

      for (a, b) in levels.iter().tuple_windows() {
        increasing = match (increasing, b > a) {
          (None, true) => Some(true),
          (None, false) => Some(false),
          (Some(true), false) | (Some(false), true) => {
            continue 'a;
          },
          _ => increasing
        };

        if b.abs_diff(*a) < 1 || b.abs_diff(*a) > 3 {
          continue 'a;
        }
      }

      safe += 1;
    }

    DayResult::Success(safe)
  }

  fn two(&self, input: &str) -> DayResult {
    let mut safe = 0;

    'a: for line in input.lines() {
      let levels = line.split_ascii_whitespace()
        .map(|level| level.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

      let mut increasing = None;

      'b: for r in 0..levels.len() {
        let mut increasing = None;
  
        for (a, b) in levels.iter()
          .enumerate()
          .filter(|(i, _)| i != &r)
          .map(|(_, v)| v)
          .tuple_windows()
        {
          increasing = match (increasing, b > a) {
            (None, true) => Some(true),
            (None, false) => Some(false),
            (Some(true), false) | (Some(false), true) => {
              continue 'a;
            },
            _ => increasing
          };
  
          if b.abs_diff(*a) < 1 || b.abs_diff(*a) > 3 {
            continue 'b;
          }
        }
  
        safe += 1;
  
        continue 'a;
      }

      for (a, b) in levels.iter().tuple_windows() {
        increasing = match (increasing, b > a) {
          (None, true) => Some(true),
          (None, false) => Some(false),
          (Some(true), false) | (Some(false), true) => {
            continue 'a;
          },
          _ => increasing
        };

        if b.abs_diff(*a) < 1 || b.abs_diff(*a) > 3 {
          continue 'a;
        }
      }

      safe += 1;
    }

    DayResult::Success(safe)
  }
}