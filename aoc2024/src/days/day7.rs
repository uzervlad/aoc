use aoc::{DayResult, DaySolver};

#[derive(Debug, Clone, Copy)]
enum Operator {
  Add,
  Mul,
  Concat
}

impl Operator {
  fn apply(&self, a: u64, b: u64) -> u64 {
    match self {
      Operator::Add => a + b,
      Operator::Mul => a * b,
      Operator::Concat => {
        let shift = b.ilog10();
        a * 10_u64.pow(shift + 1) + b
      }
    }
  }
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut total_calibration = 0;

    for line in input.lines() {
      let (total, elements) = line.split_once(": ").unwrap();
      let total = total.parse::<u64>().unwrap();
      let elements = elements.split(' ').map(|e| e.parse::<u64>().unwrap()).collect::<Vec<_>>();
      let mut operations = 0_usize;
      
      let operators = [Operator::Add, Operator::Mul];
      let max_combinations = operators.len().pow(elements.len() as u32) as usize;

      while operations < max_combinations {
        let mut result = elements[0];
        let mut current_operations = operations;

        for i in 1..elements.len() {
          let element = elements[i];
          let operator = operators[current_operations % operators.len()];
          current_operations /= operators.len();
          result = operator.apply(result, element);
        }

        if result == total {
          total_calibration += total;
          break
        }

        operations += 1;
      }
    }

    DayResult::Success(total_calibration as i64)
  }

  fn two(&self, input: &str) -> DayResult {
    let mut total_calibration = 0;

    for line in input.lines() {
      let (total, elements) = line.split_once(": ").unwrap();
      let total = total.parse::<u64>().unwrap();
      let elements = elements.split_ascii_whitespace().map(|e| e.parse::<u64>().unwrap()).collect::<Vec<_>>();
      let mut operations = 0_usize;
      
      let operators = [Operator::Add, Operator::Mul, Operator::Concat];
      let max_combinations = operators.len().pow(elements.len() as u32 - 1) as usize;

      while operations < max_combinations {
        let mut result = elements[0];
        let mut current_operations = operations;

        for i in 1..elements.len() {
          let element = elements[i];
          let operator = operators[current_operations % operators.len()];
          current_operations /= operators.len();
          result = operator.apply(result, element);
        }

        if result == total {
          total_calibration += total;
          break
        }

        operations += 1;
      }
    }

    DayResult::Success(total_calibration as i64)
  }
}