use aoc::DaySolver;

pub mod days;

pub fn get_day(day: u8) -> Box<dyn DaySolver> {
	match day {
		1 => Box::new(days::day1::Day),
		2 => Box::new(days::day2::Day),
		3 => Box::new(days::day3::Day),
		4 => Box::new(days::day4::Day),
		5 => Box::new(days::day5::Day),
		6 => Box::new(days::day6::Day),
		7 => Box::new(days::day7::Day),
		8 => Box::new(days::day8::Day),
		9 => Box::new(days::day9::Day),
		10 => Box::new(days::day10::Day),
		11 => Box::new(days::day11::Day),
		12 => Box::new(days::day12::Day),
		13 => Box::new(days::day13::Day),
		14 => Box::new(days::day14::Day),
		15 => Box::new(days::day15::Day),
		16 => Box::new(days::day16::Day),
		17 => Box::new(days::day17::Day),
		18 => Box::new(days::day18::Day),
		19 => Box::new(days::day19::Day),
		20 => Box::new(days::day20::Day),
		21 => Box::new(days::day21::Day),
		22 => Box::new(days::day22::Day),
		23 => Box::new(days::day23::Day),
		24 => Box::new(days::day24::Day),
		25 => Box::new(days::day25::Day),
		_ => panic!("Invalid day number"),
	}
}
