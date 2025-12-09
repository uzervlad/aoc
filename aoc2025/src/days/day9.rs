use std::collections::HashMap;

use aoc::{DayResult, DaySolver};

type C = i64;
type Tile = (C, C);

fn area(a: Tile, b: Tile) -> C {
  (a.0.abs_diff(b.0) + 1) as C * (a.1.abs_diff(b.1) + 1) as C
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let red_tiles: Vec<Tile> = input.lines()
      .map(|line| line.split_once(',').unwrap())
      .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
      .collect();

    let mut max_area = 0;

    for i in 0..(red_tiles.len() - 1) {
      for j in (i + 1)..red_tiles.len() {
        max_area = max_area.max(area(red_tiles[i], red_tiles[j]));
      }
    }

    DayResult::success(max_area)
  }

  fn two(&self, input: &str) -> DayResult {
    let red_tiles: Vec<Tile> = input.lines()
      .map(|line| line.split_once(',').unwrap())
      .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
      .collect();

    let mut vertical = HashMap::new();
    let mut horizontal = HashMap::new();

    for (&a, &b) in red_tiles.iter().zip(red_tiles.iter().cycle().skip(1)) {
      if a.0 == b.0 {
        vertical.entry(a.0)
          .or_insert(vec![])
          .push((a.1.min(b.1), a.1.max(b.1)));
      } else {
        horizontal.entry(a.1)
          .or_insert(vec![])
          .push((a.0.min(b.0), a.0.max(b.0)));
      }
    }

    for xs in horizontal.values_mut() {
      xs.sort();
    }
    for ys in vertical.values_mut() {
      ys.sort();
    }

    let point_in_poly = |p: Tile| {
      let mut c = false;
      for (&a, &b) in red_tiles.iter().zip(red_tiles.iter().cycle().skip(1)) {
        if (a.1 > p.1) != (b.1 > p.1) && a.0 >= p.0 {
          c = !c;
        }
        if a.1 == b.1
          && b.1 == p.1 
          && a.0.min(b.0) <= p.0
          && p.0 <= a.0.max(b.0)
        {
          return true
        }
        if a.0 == b.0
          && b.0 == p.0
          && a.1.min(b.1) <= p.1
          && p.1 <= a.1.max(b.1)
        {
          return true
        }
      }
      c
    };

    let rect_crossed = |ax: C, bx: C, ay: C, by: C| {
      for (&y, segments) in &horizontal {
        if ay < y && y < by {
          let i = segments.partition_point(|p| p.0 < ax);
          for j in i.saturating_sub(1)..=(i+1).min(segments.len() - 1) {
            let (x1, x2) = segments[j];
            if x2 > ax && x1 < bx {
              return true
            }
          }
        }
      }

      for (&x, segments) in &vertical {
        if ax < x && x < bx {
          let i = segments.partition_point(|p| p.0 < ay);
          for j in i.saturating_sub(1)..=(i+1).min(segments.len() - 1) {
            let (y1, y2) = segments[j];
            if y2 > ay && y1 < by {
              return true
            }
          }
        }
      }

      return false
    };

    let mut max_area = 0;

    for i in 0..(red_tiles.len() - 1) {
      for j in (i + 1)..red_tiles.len() {
        let a = red_tiles[i];
        let b = red_tiles[j];

        let ax = a.0.min(b.0);
        let ay = a.1.min(b.1);
        let bx = a.0.max(b.0);
        let by = a.1.max(b.1);

        let ok = point_in_poly((ax, ay))
          && point_in_poly((ax, by))
          && point_in_poly((bx, ay))
          && point_in_poly((bx, by));
        if !ok {
          continue
        }

        if rect_crossed(ax, bx, ay, by) {
          continue
        }

        max_area = max_area.max(area(a, b));
      }
    }

    DayResult::success(max_area)
  }
}