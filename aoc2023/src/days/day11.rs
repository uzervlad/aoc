use std::{collections::HashSet, ops::Range};

use aoc::{DayResult, DaySolver};

fn range(r: Range<usize>) -> Range<usize> {
  if r.is_empty() {
    r.end..r.start
  } else {
    r
  }
}

fn count_pairs(galaxies: &Vec<(usize, usize)>, empty_rows: &HashSet<usize>, empty_cols: &HashSet<usize>, expand_factor: usize) -> usize {
  (0..galaxies.len()-1)
    .map(|first| {
      (first+1..galaxies.len())
        .map(|second| {
          let first = galaxies[first];
          let second = galaxies[second];
          let raw_distance = first.0.abs_diff(second.0) + first.1.abs_diff(second.1);
          let expand_distance = range(first.0..second.0)
            .filter(|x| empty_cols.contains(x))
            .count() + range(first.1..second.1)
            .filter(|y| empty_rows.contains(y))
            .count();
          raw_distance + expand_distance * (expand_factor - 1)
        })
        .sum::<usize>()
    })
    .sum()
}

fn get_inputs(input: &str) -> (Vec<(usize, usize)>, HashSet<usize>, HashSet<usize>) {
  let (h, w) = (input.lines().count(), input.find('\n').unwrap() - 1);

  let bytes: Vec<u8> = input.lines().flat_map(|line| line.as_bytes()).map(|b| *b).collect();

  let empty_rows: HashSet<_> = (0..h).filter(|y| bytes[y*w..(y+1)*w].iter().all(|s| s == &b'.')).collect();
  let empty_cols: HashSet<_> = (0..w).filter(|x| (0..h).all(|y| bytes[y*w+x] == b'.')).collect();

  let galaxies: Vec<_> = bytes.iter()
    .enumerate()
    .filter(|(_, s)| **s == b'#')
    .map(|(i, _)| {
      (i % w, i / w)
    })
    .collect();

  (galaxies, empty_rows, empty_cols)
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let (galaxies, empty_rows, empty_cols) = get_inputs(input);

    DayResult::Success(count_pairs(&galaxies, &empty_rows, &empty_cols, 2) as i64)
  }

  fn two(&self, input: &str) -> DayResult {
    let (galaxies, empty_rows, empty_cols) = get_inputs(input);

    DayResult::Success(count_pairs(&galaxies, &empty_rows, &empty_cols, 1000000) as i64)
  }
}