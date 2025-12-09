use std::collections::HashSet;

use aoc::{DayResult, DaySolver};
use pathfinding::prelude::astar;

const SIZE: i32 = 70;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos(i32, i32);

impl Pos {
  fn distance(&self, other: &Pos) -> u32 {
    (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
  }

  fn steps(&self, bytes: &HashSet<Pos>) -> Vec<(Pos, u32)> {
    let &Pos(x, y) = self;

    vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
      .iter()
      .filter_map(|&(dx, dy)| {
        match (x.checked_add(dx), y.checked_add(dy)) {
          (Some(x), Some(y)) if (0..=SIZE).contains(&x) && (0..=SIZE).contains(&y) => match bytes.contains(&Pos(x, y)) {
            false => Some((Pos(x, y), 1)),
            true => None,
          },
          _ => None
        }
      })
      .collect()
  }
}

const TARGET: Pos = Pos(SIZE, SIZE);

pub struct Day;


impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    const SIMULATED: usize = 1024;

    let bytes = input.lines()
      .enumerate()
      .filter(|&(i, _)| i < SIMULATED)
      .map(|(_, line)| {
        let (a, b) = line.split_once(',').unwrap();

        Pos(a.parse().unwrap(), b.parse().unwrap())
      })
      .collect::<HashSet<Pos>>();

    let result = astar(&Pos(0, 0), |p| p.steps(&bytes), |p| p.distance(&TARGET), |p| *p == TARGET);

    match result {
      Some((_, steps)) => DayResult::success(steps),
      None => DayResult::Error("No path found".to_string()),
    }
  }

  fn two(&self, input: &str) -> DayResult {
    let mut bytes = HashSet::new();

    for line in input.lines() {
      let (a, b) = line.split_once(',').unwrap();
      bytes.insert(Pos(a.parse().unwrap(), b.parse().unwrap()));

      let result = astar(&Pos(0, 0), |p| p.steps(&bytes), |p| p.distance(&TARGET), |p| *p == TARGET);

      match result {
        Some(_) => continue,
        None => {
          println!();
          return DayResult::success(format!("{},{}", a, b))
        }
      }
    }

    DayResult::Error("No solution found".into())
  }
}