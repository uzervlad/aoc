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
		_ => panic!("Invalid day number"),
	}
}
