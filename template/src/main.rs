use std::{env::args, io::{stdin, stdout, Write}, time::Instant};

use aoc::DayResult;
use aoc{{year}}::days::get_day;

fn main() {
  let day: u8 = match args().nth(1) {
    Some(day) => day.parse().expect("Invalid day number"),
    None => {
      print!("Enter a day number: ");
      stdout().flush().unwrap();

      let mut input = String::new();
      let stdin = stdin();
      stdin.read_line(&mut input).unwrap();

      match input.trim().parse() {
        Ok(day) => day,
        Err(_) => {
          panic!("Invalid day number");
        },
      }
    }
  };

  let suffix = match args().nth(2) {
    Some(suffix) => format!(".{}", suffix),
    None => "".to_string(),
  };

  let input = match std::fs::read_to_string(format!("inputs/{}{}.txt", day, suffix)) {
    Ok(input) => input,
    Err(_) => {
      panic!("Unable to read input file");
    }
  };

  if day > 25 {
    panic!("Invalid day number");
  }

  let day = get_day(day);

  {
    let start = Instant::now();
    let one = day.one(&input);
    let elapsed = start.elapsed();

    match one {
      DayResult::Success(one) => {
        println!("Part one solution: {}", one);
        println!("Time: {:?}", elapsed);
      },
      DayResult::Error(error) => {
        println!("Error while solving part one: {}", error);
      },
      DayResult::Todo => {
        println!("Part one not yet implemented");
      },
    }
  }

  {
    let start = Instant::now();
    let two = day.two(&input);
    let elapsed = start.elapsed();

    match two {
      DayResult::Success(two) => {
        println!("Part two solution: {}", two);
        println!("Time: {:?}", elapsed);
      },
      DayResult::Error(error) => {
        println!("Error while solving part two: {}", error);
      },
      DayResult::Todo => {
        println!("Part two not yet implemented");
      },
    }
  }
}