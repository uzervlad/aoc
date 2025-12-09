use std::collections::HashSet;

use aoc::{DayResult, DaySolver};
use rayon::{iter::ParallelIterator, str::ParallelString};

const W: i32 = 101;
const H: i32 = 103;

struct Field {
  robots: Vec<(i32, i32, i32, i32)>,
  seconds: u64,
}

impl Field {
  fn new(input: &str) -> Self {
    let mut robots = Vec::new();

    for line in input.lines() {
      let (p, v) = line.split_once(' ').unwrap();
      let (px, py) = p[2..].split_once(',').unwrap();
      let (vx, vy) = v[2..].split_once(',').unwrap();

      let px = px.parse::<i32>().unwrap();
      let py = py.parse::<i32>().unwrap();
      let vx = vx.parse::<i32>().unwrap();
      let vy = vy.parse::<i32>().unwrap();

      robots.push((px, py, vx, vy));
    }

    Self {
      robots,
      seconds: 0
    }
  }

  fn is_unique(&self) -> bool {
    let set = self.robots.iter().map(|&(x, y, _, _)| (x, y)).collect::<HashSet<(i32, i32)>>();

    set.len() == self.robots.len()
  }

  fn step(&mut self) {
    self.robots.iter_mut().for_each(|(px, py, vx, vy)| {
      *px = (*px + *vx).rem_euclid(W);
      *py = (*py + *vy).rem_euclid(H);
    });

    self.seconds += 1;
  }
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    const SECONDS: i32 = 100;

    let result: i64 = input.par_lines()
      .filter_map(|line| {
        let (p, v) = line.split_once(' ').unwrap();
        let (px, py) = p[2..].split_once(',').unwrap();
        let (vx, vy) = v[2..].split_once(',').unwrap();

        let px = px.parse::<i32>().unwrap();
        let py = py.parse::<i32>().unwrap();
        let vx = vx.parse::<i32>().unwrap();
        let vy = vy.parse::<i32>().unwrap();

        let x = (px + vx * SECONDS).rem_euclid(W);
        let y = (py + vy * SECONDS).rem_euclid(H);

        match (x, y) {
          (0..=49, 0..=50) => Some(0),
          (51..=100, 0..=50) => Some(1),
          (0..=49, 52..=103) => Some(2),
          (51..=100, 52..=103) => Some(3),
          _ => None,
        }
      })
      .fold(|| [0_i64; 4], |mut quadrants, quadrant| {
        quadrants[quadrant] += 1;
        quadrants
      })
      .reduce(|| [0; 4], |mut reduced, quadrant| {
        for (reduced, quadrant) in reduced.iter_mut().zip(quadrant.iter()) {
          *reduced += quadrant;
        }

        reduced
      })
      .into_iter()
      .product();

    DayResult::success(result)
  }

  fn two(&self, input: &str) -> DayResult {
    let mut field = Field::new(input);

    // Apparently robots never overlap when the tree is visible
    while !field.is_unique() {
      field.step();
    }

    DayResult::success(field.seconds)
  }
}