use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{DayResult, DaySolver};

enum Device {
	Mid(Vec<u32>),
	Out,
}

const YOU: u32 = 1970239744;
const SVR: u32 = 1920365312;
const DAC: u32 = 1667326976;
const FFT: u32 = 1952867840;
const OUT: u32 = 1953853184;

fn parse_device_name(name: &str) -> u32 {
	u32::from_le_bytes([
		0,
		name.as_bytes()[0],
		name.as_bytes()[1],
		name.as_bytes()[2],
	])
}

fn parse_line(line: &str) -> (u32, Device) {
	let mut parts = line.split_whitespace();

	let name = parse_device_name(&parts.next().unwrap()[..3]);
	let devices = parts.map(parse_device_name).collect::<Vec<_>>();

	let devices = match devices[0] {
		OUT => Device::Out,
		_ => Device::Mid(devices),
	};

	(name, devices)
}

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		let map: HashMap<_, _> = input.trim().lines().map(parse_line).collect();

		if !map.contains_key(&YOU) {
			return DayResult::Error("Input has no entry device".into());
		}

		let mut stack = VecDeque::from([YOU]);
		let mut routes = 0;

		while !stack.is_empty() {
			let device = map.get(&stack.pop_front().unwrap()).unwrap();

			match device {
				Device::Mid(d) => {
					for d in d {
						stack.push_back(*d);
					}
				}
				_ => routes += 1,
			}
		}

		DayResult::success(routes)
	}

	fn two(&self, input: &str) -> DayResult {
		let map: HashMap<_, _> = input.trim().lines().map(parse_line).collect();

		if !map.contains_key(&SVR) {
			return DayResult::Error("Input has no entry device".into());
		}

		let mut routes: HashMap<(u32, u8), usize> = HashMap::new();
		let mut scheduled: HashSet<(u32, u8)> = HashSet::new();

		let mut stack = VecDeque::from([(SVR, 0)]);

		while let Some((name, mask)) = stack.pop_front() {
			if routes.contains_key(&(name, mask)) {
				continue;
			}

			let mut new_mask = mask;
			if name == DAC {
				new_mask |= 1;
			}
			if name == FFT {
				new_mask |= 2;
			}

			let device = map.get(&name).unwrap();

			match device {
				Device::Mid(d) => {
					let mut ready = true;
					for d in d {
						if !routes.contains_key(&(*d, new_mask)) {
							ready = false;

							if !scheduled.contains(&(*d, new_mask)) {
								stack.push_back((*d, new_mask));
								scheduled.insert((*d, new_mask));
							}
						}
					}

					if ready {
						let mut total = 0;
						for d in d {
							total += routes[&(*d, new_mask)];
						}
						routes.insert((name, mask), total);
					} else {
						stack.push_back((name, mask));
					}
				}
				_ => {
					if new_mask == 3 {
						routes.insert((name, mask), 1);
					} else {
						routes.insert((name, mask), 0);
					}
					continue;
				}
			}
		}

		let routes = routes[&(SVR, 0)];

		DayResult::success(routes)
	}
}
