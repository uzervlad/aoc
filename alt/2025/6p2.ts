const input = await Bun.file("./inputs/2025/6.txt").text();

const lines = input.split(/\r?\n/g);
const nums = lines.length - 1;

const lengths = [];

{
	let l = 0;
	for (const [i, ch] of Object.entries(lines[lines.length - 1])) {
		let idx = Number(i);
		if (idx > 0 && (ch === '+' || ch === '*')) {
			lengths.push(l);
			l = 0;
		}
		l++;
	}
	lengths.push(l + 1);
}

const ops = lines[lines.length - 1].split('').filter(ch => ch === '+' || ch === '*');

let result = 0;
let offset = 0;
for (const op of ops) {
	let total = op === '*' ? 1 : 0;

	for (let x = 0; x < lengths[0] - 1; x++) {
		let n = "";
		for (let y = 0; y < nums; y++) {
			n += lines[y][offset + x];
		}
		if (op === '*') {
			total *= Number(n);
		} else {
			total += Number(n);
		}
	}

	offset += lengths.shift()!;
	result += total;
}

console.log(result);