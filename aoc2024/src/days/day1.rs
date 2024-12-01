use std::collections::HashMap;

use aoc::{DayResult, DaySolver};

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut a = vec![];
    let mut b = vec![];
    
    for line in input.lines()
      .map(|line| line.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>())
    {
      a.push(line[0]);
      b.push(line[1]);
    }

    a.sort();
    b.sort();

    let mut sum = 0;
    for i in 0..a.len() {
      sum += a[i].abs_diff(b[i]);
    }

    DayResult::Success(sum as i64)
  }

  fn two(&self, input: &str) -> DayResult {
    let mut a = vec![];
    let mut b = HashMap::new();
    
    for line in input.lines()
      .map(|line| line.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>())
    {
      a.push(line[0]);
      match b.contains_key(&line[1]) {
        false => b.insert(line[1], 1),
        true => b.insert(line[1], b[&line[1]] + 1),
      };
    }

    let mut sum = 0;
    for i in 0..a.len() {
      sum += a[i] * b.get(&a[i]).unwrap_or(&0);
    }

    DayResult::Success(sum as i64)
  }
}