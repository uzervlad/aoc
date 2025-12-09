use aoc::{DayResult, DaySolver};

struct Computer<'a> {
  registers: [usize; 3],
  program: &'a [u8],
  pointer: usize,
  output: Vec<u8>,
}

impl<'a> Computer<'a> {
  fn new(registers: [usize; 3], program: &'a [u8]) -> Self {
    Self {
      registers,
      program,
      pointer: 0,
      output: Vec::new(),
    }
  }

  fn next(&mut self) -> Option<u8> {
    if self.pointer >= self.program.len() {
      return None
    }

    let v = self.program[self.pointer];
    self.pointer += 1;
    Some(v)
  }

  fn jump(&mut self, to: usize) {
    self.pointer = to;
  }

  fn step(&mut self) -> (bool, bool) {
    let Some(instruction) = self.next() else {
      return (false, false)
    };

    match instruction {
      0 => { // adv
        self.registers[0] >>= self.combo();
      },
      1 => { // blx
        self.registers[1] ^= self.literal();
      },
      2 => { // bst
        self.registers[1] = self.combo() & 0b111;
      },
      3 => { // jnz
        let pointer = self.literal();
        if self.registers[0] != 0 {
          self.jump(pointer);
        }
      },
      4 => { // bxc
        self.next();
        self.registers[1] = self.registers[1] ^ self.registers[2];
      },
      5 => { // out
        let out = self.combo() & 0b111;
        self.output.push(out as u8);
        return (true, true);
      }
      6 => { // bdv
        self.registers[1] = self.registers[0] >> self.combo();
      },
      7 => { // cdv
        self.registers[2] = self.registers[0] >> self.combo();
      }
      _ => todo!()
    }

    (true, false)
  }

  fn combo(&mut self) -> usize {
    let v = self.next().unwrap() as usize;
    match v {
      0..=3 => v,
      4..=6 => self.registers[v - 4],
      _ => unreachable!(),
    }
  }

  fn literal(&mut self) -> usize {
    self.next().unwrap() as usize
  }
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut lines = input.lines();
    let a = lines.next().unwrap()[12..].parse().unwrap();
    let b = lines.next().unwrap()[12..].parse().unwrap();
    let c = lines.next().unwrap()[12..].parse().unwrap();

    lines.next();

    let program = lines.next().unwrap()[9..]
      .split(',')
      .map(|s| s.trim().parse::<u8>().unwrap())
      .collect::<Vec<_>>();

    let mut computer = Computer::new([a, b, c], &program);

    while let (true, _) = computer.step() {}

    // println!("{}", itertools::join(computer.output.iter(), ","));

    DayResult::success(computer.output.iter().fold(0, |a, &b| a * 10 + b as i64))
  }

  fn two(&self, _: &str) -> DayResult {
    // yeah I'm not doing this shit
    DayResult::Todo
  }
}