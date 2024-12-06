use std::cmp::Ordering;

use aoc::{DayResult, DaySolver};
use itertools::Itertools;

trait Rule {
  fn check(&self, a: u32, b: u32) -> bool;
}

impl Rule for (u32, u32) {
  fn check(&self, a: u32, b: u32) -> bool {
    self.0 != b || self.1 != a
  }
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules.lines().map(|line| {
      let (a, b) = line.split_once('|').unwrap();
      
      (a.parse().unwrap(), b.parse().unwrap())
    }).collect::<Vec<_>>();

    let sum = updates.lines().map(|line| {
      let pages = line.split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

      let correct = pages.iter().combinations(2)
        .all(|w| rules.iter().all(|r| r.check(*w[0], *w[1])));

      if correct {
        pages[pages.len() / 2]
      } else {
        0
      }
    }).sum::<u32>();

    DayResult::Success(sum as i64)
  }

  fn two(&self, input: &str) -> DayResult {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules.lines().map(|line| {
      let (a, b) = line.split_once('|').unwrap();
      
      (a.parse().unwrap(), b.parse().unwrap())
    }).collect::<Vec<_>>();

    let sum = updates.lines().map(|line| {
      let mut pages = line.split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

      let correct = pages.iter().combinations(2)
        .all(|w| rules.iter().all(|r| r.check(*w[0], *w[1])));

      if !correct {
        // fix and take middle
        pages.sort_by(|&a, &b| match rules.iter().find(|&&r| r.0 == a && r.1 == b || r.0 == b && r.1 == a) {
          None => Ordering::Equal,
          Some(r) => {
            match r.0 == a {
              true => Ordering::Less,
              false => Ordering::Greater
            }
          }
        });
        pages[pages.len() / 2]
      } else {
        0
      }
    }).sum::<u32>();

    DayResult::Success(sum as i64)
  }
}