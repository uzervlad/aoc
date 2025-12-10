use aoc::{DayResult, DaySolver};

struct Box(u64, u64, u64);

impl Box {
	fn parse(s: &str) -> Self {
		let mut n = s.split(',').take(3).map(|s| s.parse().unwrap());

		Self(n.next().unwrap(), n.next().unwrap(), n.next().unwrap())
	}

	fn dist(&self, other: &Box) -> u64 {
		let dx = self.0.abs_diff(other.0);
		let dy = self.1.abs_diff(other.1);
		let dz = self.2.abs_diff(other.2);

		dx * dx + dy * dy + dz * dz
	}
}

// https://www.geeksforgeeks.org/dsa/kruskals-minimum-spanning-tree-algorithm-greedy-algo-2/
struct DSU {
	parent: Vec<usize>,
	rank: Vec<u64>,
	sets: usize,
}

impl DSU {
	fn new(n: usize) -> Self {
		Self {
			parent: (0..n).collect(),
			rank: vec![1; n],
			sets: n,
		}
	}

	fn find(&mut self, mut a: usize) -> usize {
		let mut p = a;
		while self.parent[p] != p {
			p = self.parent[p];
		}
		while self.parent[a] != a {
			let next = self.parent[a];
			self.parent[a] = p;
			a = next;
		}
		p
	}

	fn union(&mut self, a: usize, b: usize) -> bool {
		let mut ra = self.find(a);
		let mut rb = self.find(b);
		if ra == rb {
			return false;
		}
		if self.rank[ra] < self.rank[rb] {
			(ra, rb) = (rb, ra)
		}
		self.parent[rb] = ra;
		self.rank[ra] += self.rank[rb];
		self.sets -= 1;
		true
	}
}

pub struct Day;

impl DaySolver for Day {
	fn one(&self, input: &str) -> DayResult {
		const PAIRS: usize = 1000;

		let boxes: Vec<_> = input.lines().map(Box::parse).collect();

		let mut dsu = DSU::new(boxes.len());
		let mut pairs = vec![];

		for i in 0..boxes.len() - 1 {
			for j in (i + 1)..boxes.len() {
				let d = boxes[i].dist(&boxes[j]);
				pairs.push((d, i, j));
			}
		}

		pairs.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

		for &(_, i, j) in pairs.iter().take(PAIRS) {
			dsu.union(i, j);
		}

		let mut sizes = vec![0; PAIRS];
		for i in 0..boxes.len() {
			sizes[dsu.find(i)] += 1;
		}
		sizes.sort_by(|a, b| b.cmp(a));

		while sizes.len() < 3 {
			sizes.push(1);
		}

		let product = sizes[0] * sizes[1] * sizes[2];

		DayResult::success(product)
	}

	fn two(&self, input: &str) -> DayResult {
		let boxes: Vec<_> = input.lines().map(Box::parse).collect();

		let mut dsu = DSU::new(boxes.len());
		let mut pairs = vec![];

		for i in 0..boxes.len() - 1 {
			for j in (i + 1)..boxes.len() {
				let d = boxes[i].dist(&boxes[j]);
				pairs.push((d, i, j));
			}
		}

		pairs.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

		for (_, i, j) in pairs {
			let merged = dsu.union(i, j);
			if merged && dsu.sets == 1 {
				let a = boxes[i].0;
				let b = boxes[j].0;
				return DayResult::success(a * b);
			}
		}

		DayResult::Error("No result found?".into())
	}
}
