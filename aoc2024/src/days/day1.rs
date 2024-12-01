use std::iter::zip;

use aoc::{DayResult, DaySolver};

fn parse_digit(n: u8) -> u32 {
  n as u32 - 0x30
}

fn parse_num(n: &[u8]) -> u32 {
  parse_digit(n[0]) * 10000 +
  parse_digit(n[1]) * 1000 +
  parse_digit(n[2]) * 100 +
  parse_digit(n[3]) * 10 +
  parse_digit(n[4])
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut a = [0; 1000];
    let mut b = [0; 1000];
    
    for (i, (_a, _b)) in input.lines()
      .map(|line| {
        (
          parse_num(&line[0..5].as_bytes()),
          parse_num(&line[8..13].as_bytes()),
        )
      })
      .enumerate()
    {
      a[i] = _a;
      b[i] = _b;
    }

    a.sort_unstable();
    b.sort_unstable();

    let sum = zip(a.iter(), b.iter())
      .fold(0, |acc, (a, b)| acc + a.abs_diff(*b));
    DayResult::Success(sum as i64)
  }

  fn two(&self, input: &str) -> DayResult {
    let mut a = [0; 1000];
    let mut b = [0; 90000];

    for (i, (_a, _b)) in input.lines()
      .map(|line| {
        (
          parse_num(&line[0..5].as_bytes()),
          parse_num(&line[8..13].as_bytes()),
        )
      })
      .enumerate()
    {
      a[i] = _a;
      b[_b as usize - 10000] += 1;
    }

    let sum = a.iter()
      .fold(0, |acc, a| acc + a * b[*a as usize - 10000]);

    DayResult::Success(sum as i64)
  }
}