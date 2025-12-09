use std::{cmp::Ordering, collections::HashSet};

use aoc::{DayResult, DaySolver};
use fxhash::FxBuildHasher;
use itertools::Itertools;

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = {
      let mut s = HashSet::with_hasher(FxBuildHasher::default());

      for line in rules.lines() {
        let (a, b) = line.split_once('|').unwrap();
        
        s.insert((a.parse().unwrap(), b.parse().unwrap()));
      }
      
      s
    };

    let sum = updates.lines().map(|line| {
      let pages = line.split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

      let correct = pages.iter().combinations(2)
        .all(|w| !rules.contains(&(*w[1], *w[0])));

      if correct {
        pages[pages.len() / 2]
      } else {
        0
      }
    }).sum::<u32>();

    DayResult::success(sum)
  }

  fn two(&self, input: &str) -> DayResult {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = {
      let mut s = HashSet::with_hasher(FxBuildHasher::default());

      for line in rules.lines() {
        let (a, b) = line.split_once('|').unwrap();
        
        s.insert((a.parse().unwrap(), b.parse().unwrap()));
      }
      
      s
    };

    let sum = updates.lines().map(|line| {
      let mut pages = line.split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

        let correct = pages.iter().combinations(2)
          .all(|w| !rules.contains(&(*w[1], *w[0])));

      if !correct {
        pages.sort_unstable_by(|&a, &b| match (rules.contains(&(a, b)), rules.contains(&(b, a))) {
          (true, false) => Ordering::Less,
          (false, true) => Ordering::Greater,
          _ => Ordering::Equal
        });
        pages[pages.len() / 2]
      } else {
        0
      }
    }).sum::<u32>();

    DayResult::success(sum)
  }
}