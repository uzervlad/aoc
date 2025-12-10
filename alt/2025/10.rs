use std::{env::args, fs};

use z3::{Optimize, SatResult, ast::{Bool, Int}};

fn part_one(lights: &[bool], buttons: &[Vec<usize>]) -> usize {
	let opt = Optimize::new();

	let press_vars: Vec<_> = (0..buttons.len())
		.map(|i| Bool::new_const(format!("p{}", i)))
		.collect();

	for light in 0..lights.len() {
		let toggled: Vec<_> = buttons.iter()
			.enumerate()
			.filter(|&(_, btn)| btn.contains(&light))
			.map(|(i, _)| press_vars[i].clone())
			.collect();

		let xor = if toggled.is_empty() {
			Bool::from_bool(false)
		} else {
			toggled.iter()
				.fold(Bool::from_bool(false), |acc, b| acc.xor(b))
		};

		let target_bool = Bool::from_bool(lights[light]);
		opt.assert(&xor.eq(&target_bool));
	}

	let zero = Int::from_u64(0);
	let one = Int::from_u64(1);

	let presses: Vec<_> = press_vars.iter().map(|p| p.ite(&one, &zero)).collect();
	let total = Int::add(&presses);
	opt.minimize(&total);

	match opt.check(&[]) {
		SatResult::Sat => {
			let model = opt.get_model().unwrap();
			press_vars.iter()
				.filter(|&p| model.eval(p, true).unwrap().as_bool().unwrap_or(false))
				.count()
		},
		_ => panic!("fuck")
	}
}

fn part_two(joltage: &[u64], buttons: &[Vec<usize>]) -> usize {
	let opt = Optimize::new();

	let press_vars: Vec<_> = (0..buttons.len())
		.map(|i| Int::new_const(format!("p{}", i)))
		.collect();

	for v in &press_vars {
		opt.assert(&v.ge(0));
	}

	for j in 0..joltage.len() {
		let toggled: Vec<_> = buttons.iter()
			.enumerate()
			.filter(|&(_, btn)| btn.contains(&j))
			.map(|(i, _)| press_vars[i].clone())
			.collect();

		let total = Int::add(&toggled);
		let target = Int::from_u64(joltage[j]);
		opt.assert(&total.eq(&target));
	}

	let total = Int::add(&press_vars);
	opt.minimize(&total);

	match opt.check(&[]) {
		SatResult::Sat => {
			let model = opt.get_model().unwrap();
			press_vars.iter()
				.map(|p| model.eval(p, true).unwrap().as_u64().unwrap_or_default())
				.sum::<u64>() as _
		},
		_ => panic!("no solution found")
	}
}

fn parse_line(line: &str) -> (Vec<bool>, Vec<Vec<usize>>, Vec<u64>) {
	let mut parts = line.split_whitespace();

	let lights = parts.next().unwrap()
		.as_bytes().iter()
		.skip(1).take_while(|&&b| b != b']')
		.map(|&b| b == b'#')
		.collect();

	let mut buttons = vec![];

	let mut joltage = None;
	
	for part in parts {
		if part.starts_with('{') {
			joltage = Some(part
				.trim_start_matches('{')
				.trim_end_matches('}')
				.split(',')
				.map(str::parse)
				.map(Result::unwrap)
				.collect());
			break;
		}

		buttons.push(
			part
				.trim_start_matches('(')
				.trim_end_matches(')')
				.split(',')
				.map(str::parse)
				.map(Result::unwrap)
				.collect()
		);
	}

	(lights, buttons, joltage.unwrap())
}

fn main() {
	let filename = args().skip(1).next().unwrap();

	let file = fs::read_to_string(filename).unwrap();

	let mut one = 0;
	let mut two = 0;

	for line in file.trim().lines() {
		let (lights, buttons, joltage) = parse_line(line);
		one += part_one(&lights, &buttons);
		two += part_two(&joltage, &buttons);
	}

	println!("{}", one);
	println!("{}", two);
}