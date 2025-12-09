use std::{iter::Peekable, str::Bytes};

use aoc::{DayResult, DaySolver};

struct OperationIterator<'a> {
  input: Peekable<Bytes<'a>>,
  state: ParseState,
}

impl<'a> OperationIterator<'a> {
  fn new(input: &'a str) -> Self {
    Self { input: input.bytes().peekable(), state: ParseState::ParsingName }
  }
}

enum Operation {
  Multiply(i64, i64),
  Do,
  Dont,
}

enum ParseState {
  ParsingName,
  ParsingA,
  ParsingB(i64),
}

impl<'a> Iterator for OperationIterator<'a> {
  type Item = Operation;

  fn next(&mut self) -> Option<Self::Item> {
    'a: loop {
      match self.state {
        ParseState::ParsingName => {
          match self.input.peek() {
            Some(b'm') => {
              self.input.next();
              match self.input.peek() {
                Some(b'u') => {
                  self.input.next();
                  match self.input.peek() {
                    Some(b'l') => {
                      self.input.next();
                      match self.input.peek() {
                        Some(b'(') => {
                          self.input.next();
                          self.state = ParseState::ParsingA;
                          continue
                        },
                        None => break,
                        _ => continue
                      }
                    },
                    None => break,
                    _ => continue
                  }
                },
                None => break,
                _ => continue
              }
            },
            Some(b'd') => {
              self.input.next();
              match self.input.peek() {
                Some(b'o') => {
                  self.input.next();
                  match self.input.peek() {
                    Some(b'n') => {
                      self.input.next();
                      match self.input.peek() {
                        Some(b'\'') => {
                          self.input.next();
                          match self.input.peek() {
                            Some(b't') => {
                              self.input.next();
                              match self.input.peek() {
                                Some(b'(') => {
                                  self.input.next();
                                  match self.input.peek() {
                                    Some(b')') => {
                                      self.input.next();
                                      return Some(Operation::Dont)
                                    },
                                    None => break,
                                    _ => continue
                                  }
                                },
                                None => break,
                                _ => continue
                              }
                            },
                            None => break,
                            _ => continue
                          }
                        },
                        None => break,
                        _ => continue
                      }
                    },
                    Some(b'(') => {
                      self.input.next();
                      match self.input.peek() {
                        Some(b')') => {
                          self.input.next();
                          return Some(Operation::Do)
                        },
                        None => break,
                        _ => continue
                      }
                    },
                    None => break,
                    _ => continue
                  }
                },
                None => break,
                _ => continue
              }
            },
            None => break,
            _ => {
              self.input.next();
              continue
            }
          }
        },
        ParseState::ParsingA => {
          let mut a = 0;
          while let Some(ch) = self.input.peek() {
            if ch.is_ascii_digit() {
              a *= 10;
              a += (self.input.next().unwrap() - 0x30) as i64;
            } else if *ch == b',' {
              self.input.next();
              self.state = ParseState::ParsingB(a);
              continue 'a
            } else {
              self.state = ParseState::ParsingName;
              continue 'a
            }
          }
          return None
        }
        ParseState::ParsingB(a) => {
          let mut b = 0;
          while let Some(ch) = self.input.peek() {
            if ch.is_ascii_digit() {
              b *= 10;
              b += (self.input.next().unwrap() - 0x30) as i64;
            } else if *ch == b')' {
              self.input.next();
              self.state = ParseState::ParsingName;
              return Some(Operation::Multiply(a, b))
            } else {
              self.state = ParseState::ParsingName;
              continue 'a
            }
          }
          return None
        }
      }
    }

    None
  }
}

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let iter = OperationIterator::new(input);

    let mut sum = 0;

    for op in iter {
      match op {
        Operation::Multiply(a, b) => sum += a * b,
        _ => (),
      }
    }

    DayResult::success(sum)
  }

  fn two(&self, input: &str) -> DayResult {
    let iter = OperationIterator::new(input);

    let mut sum = 0;
    let mut yes = true;

    for op in iter {
      match op {
        Operation::Multiply(a, b) if yes => sum += a * b,
        Operation::Do => yes = true,
        Operation::Dont => yes = false,
        _ => {}
      }
    }

    DayResult::success(sum)
  }
}