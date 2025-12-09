use aoc::{DayResult, DaySolver};

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut dial = 50_u32;
    let mut zeros = 0;
    for line in input.lines() {
      let rot = line[1..].parse::<u32>().unwrap();
      match line.as_bytes()[0] {
        b'L' => if rot > dial {
          dial = (100 - (rot - dial) % 100) % 100
        } else {
          dial -= rot;
        },
        b'R' => dial = (dial + rot) % 100,
        _ => unreachable!()
      }
      if dial == 0 {
        zeros += 1;
      }
    }
    DayResult::success(zeros)
  }

  fn two(&self, input: &str) -> DayResult {
    let mut dial = 50_u32;
    let mut zeros = 0;
    for line in input.lines() {
      let rot = line[1..].parse::<u32>().unwrap();
      // this is so fucking ugly :sob:
      match line.as_bytes()[0] {
        b'L' => if rot >= dial {
          zeros += (rot - dial) as i64 / 100 + 1;
          if dial == 0 {
            zeros -= 1;
          }
          dial = (100 - (rot - dial) % 100) % 100
        } else {
          dial -= rot;
        },
        b'R' => {
          dial += rot;
          zeros += dial as i64 / 100;
          dial %= 100;
        },
        _ => unreachable!()
      }
    }
    DayResult::success(zeros)
  }
}