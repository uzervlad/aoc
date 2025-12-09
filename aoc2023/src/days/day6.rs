use aoc::{DayResult, DaySolver};

pub struct Day;

impl DaySolver for Day {
  fn one(&self, input: &str) -> DayResult {
    let mut lines = input.lines();

    let times = lines.next().unwrap()
      .split_once(':').unwrap()
      .1.split(' ').filter(|s| !s.is_empty())
      .flat_map(str::parse);

    let distances = lines.next().unwrap()
      .split_once(':').unwrap()
      .1.split(' ').filter(|s| !s.is_empty())
      .flat_map(str::parse);

    let races = times.zip(distances);

    DayResult::success(
      races.map(|r| {
        for i in 1..r.0 {
          if i * (r.0 - i) > r.1 {
            return r.0 - i * 2 + 1
          }
        }
        0
      }).product::<i32>()
    )
  }

  fn two(&self, input: &str) -> DayResult {
    let mut lines = input.lines();

    let time = lines.next().unwrap()
      .split_once(':').unwrap()
      .1.replace(' ', "").trim().parse::<u64>().unwrap();

    let distance = lines.next().unwrap()
      .split_once(':').unwrap()
      .1.replace(' ', "").trim().parse::<u64>().unwrap();

    // solve quadratic equation
    // -x^2 + time*x - distance = 0
    let root = {
      let b = time as f64;
      let c = -(distance as f64);
      let d = b * b + 4. * c;
      let sq = d.sqrt();
      let (same_sign, diff_sign) = (-b - sq, -b + sq);
      let cx2 = 2. * c;
      (cx2 / same_sign).min(cx2 / diff_sign)
    };

    DayResult::success(time - root.ceil().max(0.) as u64 * 2 + 1)
  }
}