#[derive(Debug)]
pub enum DayResult {
  Success(i64),
  Error(String),
  Todo,
}

pub trait DaySolver {
  fn one(&self, input: &str) -> DayResult;
  fn two(&self, input: &str) -> DayResult;
}