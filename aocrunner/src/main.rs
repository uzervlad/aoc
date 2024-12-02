use std::{env, io::{stdin, stdout, Write}, time::{Duration, Instant}};

use aoc::{DayResolver, DayResult, DaySolver};
use clap::{command, Parser};
use reqwest::{blocking::ClientBuilder, header::HeaderMap};

fn submit_answer(year: u16, day: u8, level: u8, answer: i64) {
  if dotenvy::from_path(".env").is_err() {
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

  let Ok(client) = ClientBuilder::new()
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

  let Ok(response) = client.post(format!("https://adventofcode.com/{}/day/{}/answer", year, day))
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

fn get_day_resolver(year: u16) -> DayResolver {
  match year {
    2023 => aoc2023::get_day,
    2024 => aoc2024::get_day,
    _ => unreachable!(),
  }
}

#[derive(Parser)]
#[command()]
struct Cli {
  #[arg(required = true)]
  year: u16,
  #[arg(required = true)]
  day: u8,
  suffix: Option<String>,

  #[arg(long)]
  bench: bool,
  #[arg(long, default_value = "20")]
  runs: usize,
}

fn benchmark_values(mut runs: Vec<u128>) -> (u128, u128, usize) {
  let mean = runs.iter().sum::<u128>() / runs.len() as u128;

  let variance = runs.iter()
    .map(|r| r.abs_diff(mean).pow(2))
    .sum::<u128>() / runs.len() as u128;

  let stddev = (variance as f64).sqrt();

  let a = runs.len();
  runs.retain(|r| (r.abs_diff(mean) / stddev as u128) <= 3);
  
  let removed = a - runs.len();

  if removed > 0 {
    let (mean, stddev, removed_new) = benchmark_values(runs);
    (mean, stddev, removed + removed_new)
  } else {
    (mean, stddev as u128, removed)
  }
}

fn benchmark(solver: Box<dyn DaySolver>, input: &str, runs_count: usize) {
  println!("Running {} times", runs_count);

  {
    let mut runs = vec![];
    for _ in 0..runs_count {
      let start = Instant::now();
      let _ = solver.one(&input);
      let elapsed = start.elapsed();

      runs.push(elapsed.as_nanos());
    }

    let (mean, stddev, removed) = benchmark_values(runs);

    let mean = Duration::from_nanos(mean as u64);
    let stddev = Duration::from_nanos(stddev as u64);

    println!("Part one:");
    println!("Mean: {:?}", mean);
    println!("Stddev: {:?}", stddev);
    println!("Removed {} outliers", removed);
  }

  {
    let mut runs = vec![];
    for _ in 0..runs_count {
      let start = Instant::now();
      let _ = solver.two(&input);
      let elapsed = start.elapsed();

      runs.push(elapsed.as_nanos());
    }

    let (mean, stddev, removed) = benchmark_values(runs);

    let mean = Duration::from_nanos(mean as u64);
    let stddev = Duration::from_nanos(stddev as u64);

    println!("Part two:");
    println!("Mean: {:?}", mean);
    println!("Stddev: {:?}", stddev);
    println!("Removed {} outliers", removed);
  }
}

fn run(year: u16, day: u8, solver: Box<dyn DaySolver>, input: &str) {
  {
    let start = Instant::now();
    let one = solver.one(&input);
    let elapsed = start.elapsed();

    match one {
      DayResult::Success(one) => {
        println!("Part one solution: {}", one);
        println!("Time: {:?}", elapsed);

        if prompt_submit() {
          submit_answer(year, day, 1, one);
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
          submit_answer(year, day, 2, two);
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

fn main() {
  let args = Cli::parse();

  let resolver = get_day_resolver(args.year);

  let suffix = match args.suffix {
    Some(suffix) => format!(".{}", suffix),
    None => "".to_string(),
  };

  let filename = format!("inputs/{}/{}{}.txt", args.year, args.day, suffix);

  let input = match std::fs::read_to_string(filename) {
    Ok(input) => input,
    Err(_) => {
      println!("Unable to read input file");
      return;
    }
  };

  if args.day > 25 {
    println!("Invalid day number");
    return;
  }

  let solver = resolver(args.day);

  if args.bench {
    benchmark(solver, &input, args.runs);
  } else {
    run(args.year, args.day, solver, &input);
  }  
}