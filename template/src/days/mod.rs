use aoc::DaySolver;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn get_day(day: u8) -> Box<dyn DaySolver> {
  match day {
    1 => Box::new(day1::Day),
    2 => Box::new(day2::Day),
    3 => Box::new(day3::Day),
    4 => Box::new(day4::Day),
    5 => Box::new(day5::Day),
    6 => Box::new(day6::Day),
    7 => Box::new(day7::Day),
    8 => Box::new(day8::Day),
    9 => Box::new(day9::Day),
    10 => Box::new(day10::Day),
    11 => Box::new(day11::Day),
    12 => Box::new(day12::Day),
    13 => Box::new(day13::Day),
    14 => Box::new(day14::Day),
    15 => Box::new(day15::Day),
    16 => Box::new(day16::Day),
    17 => Box::new(day17::Day),
    18 => Box::new(day18::Day),
    19 => Box::new(day19::Day),
    20 => Box::new(day20::Day),
    21 => Box::new(day21::Day),
    22 => Box::new(day22::Day),
    23 => Box::new(day23::Day),
    24 => Box::new(day24::Day),
    25 => Box::new(day25::Day),
    _ => panic!("Invalid day number"),
  }
}