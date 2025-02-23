use aoc::{DayResult, DaySolver};
use hashbrown::HashMap;

fn solve_node(start: &str, nodes: &HashMap<&str, (&str, &str)>, steps: &Vec<char>) -> u64 {
  let l = steps.len() as u64;
  let mut current_node = start;
  let mut taken: u64 = 0;
  loop {
    let step = steps[(taken % l) as usize];

    let branches = nodes.get(&current_node).unwrap();
    match step {
      'L' => current_node = branches.0,
      'R' => current_node = branches.1,
      _ => (),
    }

    taken += 1;

    if current_node.as_bytes()[2] == b'Z' {
      break
    }
  }

  taken
}

fn get_inputs(input: &str) -> (HashMap<&str, (&str, &str)>, Vec<char>) {
  let mut lines = input.lines();

  let steps: Vec<char> = lines.next().unwrap().chars().collect();

  let nodes: HashMap<&str, (&str, &str)> = lines
    .filter(|l| !l.is_empty())
    .map(|line| {
      let node = &line[0..3];
      let branch_a = &line[7..10];
      let branch_b = &line[12..15];
      (node, (branch_a, branch_b))
    })
    .collect();

  (nodes, steps)
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let (nodes, steps) = get_inputs(input);

    DayResult::Success(solve_node("AAA", &nodes, &steps) as i64)
  }

  fn two(&self, input: &str) -> DayResult {
    let (nodes, steps) = get_inputs(input);
    
    let factors: Vec<u64> = nodes.keys()
      .filter(|n| n.as_bytes()[2] == b'A')
      .map(|n| solve_node(*n, &nodes, &steps))
      .collect();

    let mut lcm = num_integer::lcm(factors[0], factors[1]);
    for i in 2..factors.len() {
      lcm = num_integer::lcm(lcm, factors[i]);
    }

    DayResult::Success(lcm as i64)
  }
}