use std::collections::{HashMap, HashSet};

use aoc::{DayResult, DaySolver};
use fxhash::FxBuildHasher;

type Position = (i32, i32);

trait DirectionTrait {
  fn out_of_bounds(&self, size: i32) -> bool;
}

impl DirectionTrait for Position {
  fn out_of_bounds(&self, size: i32) -> bool {
    self.0 < 0 || self.0 >= size || self.1 < 0 || self.1 >= size
  }
}

fn antinode(a: Position, b: Position) -> Position {
  (b.0 + b.0 - a.0, b.1 + b.1 - a.1)
}

struct AntinodeIterator {
  pos: Position,
  dx: i32,
  dy: i32,
  size: i32,
}

impl AntinodeIterator {
  fn new(a: Position, b: Position, size: i32) -> AntinodeIterator {
    AntinodeIterator {
      pos: b,
      dx: b.0 - a.0,
      dy: b.1 - a.1,
      size
    }
  }
}

impl Iterator for AntinodeIterator {
  type Item = Position;

  fn next(&mut self) -> Option<Self::Item> {
    let result = self.pos;
    if result.out_of_bounds(self.size) {
      None
    } else {
      self.pos = (self.pos.0 + self.dx, self.pos.1 + self.dy);
      Some(result)
    }
  }
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut nodes = HashMap::with_hasher(FxBuildHasher::default());

    let mut size = 0;

    let mut x = 0;
    let mut y = 0;

    for &b in input.as_bytes() {
      match b {
        b'.' => {
          x += 1;
        },
        b'\n' => {
          size = x.max(size);
          y += 1;
          x = 0;
        },
        b'\r' => {},
        b => {
          nodes.entry(b).or_insert(Vec::with_capacity(100)).push((x, y));
          x += 1;
        }
      }
    }

    let mut antinodes = HashSet::with_hasher(FxBuildHasher::default());

    for f_nodes in nodes.values() {
      for i in 0..f_nodes.len() {
        for j in 0..i {
          let a_node = antinode(f_nodes[i], f_nodes[j]);
          if !a_node.out_of_bounds(size) {
            antinodes.insert(a_node);
          }
          let a_node = antinode(f_nodes[j], f_nodes[i]);
          if !a_node.out_of_bounds(size) {
            antinodes.insert(a_node);
          }
        }
      }
    }

    DayResult::success(antinodes.len())
  }

  fn two(&self, input: &str) -> DayResult {
    let mut nodes = HashMap::with_hasher(FxBuildHasher::default());

    let mut size = 0;

    let mut x = 0;
    let mut y = 0;

    for &b in input.as_bytes() {
      match b {
        b'.' => {
          x += 1;
        },
        b'\n' => {
          size = x.max(size);
          y += 1;
          x = 0;
        },
        b'\r' => {},
        b => {
          nodes.entry(b).or_insert(Vec::new()).push((x, y));
          x += 1;
        }
      }
    }

    let mut antinodes = HashSet::with_hasher(FxBuildHasher::default());

    for f_nodes in nodes.values() {
      for i in 0..f_nodes.len() {
        for j in 0..i {
          for a in AntinodeIterator::new(f_nodes[i], f_nodes[j], size) {
            antinodes.insert(a);
          }
          for a in AntinodeIterator::new(f_nodes[j], f_nodes[i], size) {
            antinodes.insert(a);
          }
        }
      }
    }

    DayResult::success(antinodes.len())
  }
}