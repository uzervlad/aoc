use std::collections::HashMap;

use aoc::{DayResult, DaySolver};

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut a = vec![];
    let mut b = vec![];
    
    for (_a, _b) in input.lines()
      .filter_map(|line| line.split_once("   "))
      .map(|(_a, _b)| (_a.parse::<u32>().unwrap(), _b.parse::<u32>().unwrap()))
    {
      a.push(_a);
      b.push(_b);
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut sum = 0;
    for i in 0..a.len() {
      sum += a[i].abs_diff(b[i]);
    }

    DayResult::Success(sum as i64)
  }

  fn two(&self, input: &str) -> DayResult {
    let mut a = vec![];
    let mut b = HashMap::new();
    
    for (_a, _b) in input.lines()
      .filter_map(|line| line.split_once("   "))
      .map(|(_a, _b)| (_a.parse::<u32>().unwrap(), _b.parse::<u32>().unwrap()))
    {
      a.push(_a);
      *b.entry(_b).or_insert(0) += 1;
    }

    let mut sum = 0;
    for i in 0..a.len() {
      sum += a[i] * b.get(&a[i]).unwrap_or(&0);
    }

    DayResult::Success(sum as i64)
  }
}