use std::collections::HashMap;

use aoc::{DayResult, DaySolver};

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut a = Vec::with_capacity(1000);
    let mut b = Vec::with_capacity(1000);
    
    for (_a, _b) in input.lines()
      .filter_map(|line| line.split_once("   "))
      .map(|(_a, _b)| (_a.parse::<u32>().unwrap(), _b.parse::<u32>().unwrap()))
    {
      a.push(_a);
      b.push(_b);
    }

    a.sort_unstable();
    b.sort_unstable();

    let sum = (0..a.len())
      .map(|i| a[i].abs_diff(b[i]))
      .sum::<u32>();

    DayResult::Success(sum as i64)
  }

  fn two(&self, input: &str) -> DayResult {
    let mut a = Vec::with_capacity(1000);
    let mut b = HashMap::with_capacity(1000);
    
    for (_a, _b) in input.lines()
      .filter_map(|line| line.split_once("   "))
      .map(|(_a, _b)| (_a.parse::<u32>().unwrap(), _b.parse::<u32>().unwrap()))
    {
      a.push(_a);
      *b.entry(_b).or_insert(0) += 1;
    }

    let sum = (0..a.len())
      .map(|i| a[i] * b.get(&a[i]).unwrap_or(&0))
      .sum::<u32>();

    DayResult::Success(sum as i64)
  }
}