use std::str::FromStr;

use aoc::{DayResult, DaySolver};

enum Item {
  Number(u64),
  Operation(u8),
}

impl FromStr for Item {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s.as_bytes()[0] {
      b'+' => Self::Operation(b'+'),
      b'*' => Self::Operation(b'*'),
      _ => Self::Number(s.parse().unwrap()),
    })
  }
}

trait Fold {
  fn fold(&self, op: u8) -> u64;
}

impl Fold for Vec<u64> {
  fn fold(&self, op: u8) -> u64 {
    let init = match op {
      b'+' => 0,
      b'*' => 1,
      _ => unreachable!()
    };
    self.iter().fold(init, |acc, r| match op {
      b'+' => acc + *r,
      b'*' => acc * *r,
      _ => unreachable!()
    })
  }
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut stuff: Vec<Vec<_>> = vec![];

    let mut sum = 0;

    for line in input.lines() {
      for (col, item) in line.split_whitespace().enumerate() {
        let item = Item::from_str(item).unwrap();

        match item {
          Item::Number(num) => {
            if let Some(list) = stuff.get_mut(col) {
              list.push(num);
            } else {
              stuff.push(vec![num]);
            }
          },
          Item::Operation(op) => {
            sum += stuff[col].fold(op);
          }
        }
      }
    }

    DayResult::success(sum)
  }

  fn two(&self, _: &str) -> DayResult {
    DayResult::Todo
  }
}