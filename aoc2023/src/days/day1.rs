use aoc::{DayResult, DaySolver};

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut calibration_value = 0;
  
    for line in input.lines() {
      let numbers = line.chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();
      let a = numbers.first().unwrap();
      let b = numbers.last().unwrap();
      calibration_value += a * 10 + b;
    }
  
    DayResult::Success(calibration_value as i64)
  }
  
  fn two(&self, input: &str) -> DayResult {
    let mut calibration_value = 0u32;
  
    for line in input.lines() {
      let numbers = line.chars()
        .enumerate()
        .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)))
        .collect::<Vec<(usize, u32)>>();
      let str_first_indices = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter()
        .map(|s| line.find(s))
        .collect::<Vec<Option<usize>>>();
      let str_last_indices = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter()
        .map(|s| line.rfind(s))
        .collect::<Vec<Option<usize>>>();
      let a = {
        let mut i = line.len();
        let mut a = 0;
        for j in 1..=9 {
          if let Some(k) = str_first_indices[j - 1] {
            if i > k {
              i = k;
              a = j;
            }
          }
        }
        match numbers.first() {
          Some((j, a)) if *j < i => *a,
          _ => a as u32
        }
      };
      let b = {
        let mut i = 0;
        let mut b = 0;
        for j in 1..=9 {
          if let Some(k) = str_last_indices[j - 1] {
            if i < k {
              i = k;
              b = j;
            }
          }
        }
        match numbers.last() {
          Some((j, b)) if *j >= i => *b,
          _ => b as u32
        }
      };
      calibration_value += a * 10 + b;
    }
  
    DayResult::Success(calibration_value as i64)
  }
}