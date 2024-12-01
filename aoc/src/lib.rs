use std::{env::{self, args}, io::{stdin, stdout, Write}, time::Instant};

use reqwest::header::HeaderMap;

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

pub type DayResolver = fn(u8) -> Box<dyn DaySolver>;

fn submit_answer(day: u8, level: u8, answer: i64) {
  if dotenvy::from_path("../.env").is_err() {
    println!("Unable to read .env");
    return;
  }

  let session = match env::var("AOC_SESSION") {
    Ok(session) => session,
    Err(_) => {
      println!("No session cookie provided in .env");
      return;
    }
  };

  let Ok(client) = reqwest::blocking::ClientBuilder::new()
    .default_headers({
      let mut headers = HeaderMap::new();
      headers.append("Cookie", format!("session={}", session).parse().unwrap());
      headers
    })
    .build() else
  {
    println!("Unable to create reqwest client");
    return;
  };

  let params = [
    ("level", level.to_string()),
    ("answer", answer.to_string()),
  ];

  let Ok(response) = client.post(format!("https://adventofcode.com/2022/day/{}/answer", day))
    .form(&params)
    .send() else
  {
    println!("Unable to submit answer");
    return;
  };

  let Ok(data) = response.text() else {
    println!("Unable to read response");
    return;
  };

  if data.contains("not the right answer") {
    println!("Incorrect answer");
    return;
  }
}

fn prompt_submit() -> bool {
  print!("Submit answer? [y/N] ");
  stdout().flush().unwrap();

  let mut input = String::new();
  let stdin = stdin();
  stdin.read_line(&mut input).unwrap();

  input.trim().to_lowercase() == "y"
}

pub fn main(resolver: DayResolver) {
  let day: u8 = match args().nth(1) {
    Some(day) => match day.parse() {
      Ok(day) => day,
      Err(_) => {
        println!("Invalid day number");
        return;
      }
    },
    None => {
      print!("Enter a day number: ");
      stdout().flush().unwrap();

      let mut input = String::new();
      let stdin = stdin();
      stdin.read_line(&mut input).unwrap();

      match input.trim().parse() {
        Ok(day) => day,
        Err(_) => {
          println!("Invalid day number");
          return;
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
      println!("Unable to read input file");
      return;
    }
  };

  if day > 25 {
    println!("Invalid day number");
    return;
  }

  let solver = resolver(day);

  {
    let start = Instant::now();
    let one = solver.one(&input);
    let elapsed = start.elapsed();

    match one {
      DayResult::Success(one) => {
        println!("Part one solution: {}", one);
        println!("Time: {:?}", elapsed);

        if prompt_submit() {
          submit_answer(day, 1, one);
        }
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
    let two = solver.two(&input);
    let elapsed = start.elapsed();

    match two {
      DayResult::Success(two) => {
        println!("Part two solution: {}", two);
        println!("Time: {:?}", elapsed);

        if prompt_submit() {
          submit_answer(day, 2, two);
        }
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