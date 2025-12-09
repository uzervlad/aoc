use aoc::{DayResult, DaySolver};

pub struct Day;

#[derive(Default)]
struct MaxTable {
  red: u32,
  green: u32,
  blue: u32,
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

impl MaxTable {
  fn is_possible(&self) -> bool {
    self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
  }
}

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut sum = 0;

    for line in input.lines() {
      let mut max_table = MaxTable::default();
      let (id, rounds) = line.split_once(": ").unwrap();
      let rounds = rounds.split("; ");
      for round in rounds {
        let cubes = round.split(", ");
        for cube in cubes {
          let (count, color) = cube.split_once(" ").unwrap();
          match color {
            "red" => max_table.red = max_table.red.max(str::parse(count).unwrap()),
            "green" => max_table.green = max_table.green.max(str::parse(count).unwrap()),
            "blue" => max_table.blue = max_table.blue.max(str::parse(count).unwrap()),
            _ => (),
          }
        }
      }
      if max_table.is_possible() {
        sum += str::parse::<u32>(&id[5..]).unwrap();
      }
    }

    DayResult::success(sum)
  }

  fn two(&self, input: &str) -> DayResult {
    let mut sum = 0;

    for line in input.lines() {
      let mut max_table = MaxTable::default();
      let (_, rounds) = line.split_once(": ").unwrap();
      let rounds = rounds.split("; ");
      for round in rounds {
        let cubes = round.split(", ");
        for cube in cubes {
          let (count, color) = cube.split_once(" ").unwrap();
          match color {
            "red" => max_table.red = max_table.red.max(str::parse(count).unwrap()),
            "green" => max_table.green = max_table.green.max(str::parse(count).unwrap()),
            "blue" => max_table.blue = max_table.blue.max(str::parse(count).unwrap()),
            _ => (),
          }
        }
      }
      sum += max_table.red * max_table.green * max_table.blue
    }

    DayResult::success(sum)
  }
}