use aoc::{DayResult, DaySolver};
use regex::Regex;

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;

    for cap in re.captures_iter(input) {
      let a = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
      let b = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
      sum += a * b;
    }

    DayResult::Success(sum)
  }

  fn two(&self, input: &str) -> DayResult {
    let re = Regex::new(r"(?P<mul>mul\((\d+),(\d+)\))|(?P<do>do\(\))|(?P<dont>don't\(\))").unwrap();

    let mut sum = 0;
    let mut yes = true;

    for cap in re.captures_iter(input) {
      if let Some(_) = cap.name("do") {
        yes = true;
        continue;
      } else if let Some(_) = cap.name("dont") {
        yes = false;
        continue;
      } else if let Some(_) = cap.name("mul") {
        if !yes {
          continue;
        }

        let a = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let b = cap.get(3).unwrap().as_str().parse::<i64>().unwrap();
        sum += a * b;
        continue;
      }
    }

    DayResult::Success(sum)
  }
}