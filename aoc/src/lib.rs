use std::fmt::Display;

use colored::Colorize as _;

#[derive(Debug)]
pub enum DayResult {
	Success(DayResultValue),
	Error(String),
	Todo,
	Note(String),
}

impl DayResult {
	pub fn success<V: Into<DayResultValue>>(value: V) -> Self {
		Self::Success(value.into())
	}
}

impl Display for DayResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DayResult::Success(result) => {
				write!(f, "Solution: {}", format!("{}", result).yellow())
			}
			DayResult::Error(error) => {
				write!(f, "{}", format!("Error: {}", error).bright_red())
			}
			DayResult::Todo => {
				write!(f, "{}", "Not implemented yet".bright_black())
			}
			DayResult::Note(note) => {
				write!(f, "{}", note.bright_purple())
			}
		}
	}
}

macro_rules! impl_result {
  ($($i:ident -> $t:ty),+) => {
    #[derive(Debug)]
    pub enum DayResultValue {
      $($i($t),)+
    }

    $(impl From<$t> for DayResultValue {
      fn from(value: $t) -> Self {
        Self::$i(value)
      }
    })+

    impl Display for DayResultValue {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $(Self::$i(v) => write!(f, "{}", v),)+
        }
      }
    }
  };
}

impl_result!(
  U8 -> u8,
  U16 -> u16,
  U32 -> u32,
  U64 -> u64,
  U128 -> u128,
  Usize -> usize,
  I8 -> i8,
  I16 -> i16,
  I32 -> i32,
  I64 -> i64,
  I128 -> i128,
  Isize -> isize,
  String -> String
);

pub trait DaySolver {
	fn one(&self, input: &str) -> DayResult;
	fn two(&self, input: &str) -> DayResult;
}

pub type DayResolver = fn(u8) -> Box<dyn DaySolver>;
