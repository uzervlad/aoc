export async function fetchInput(year: string, day: string) {
  if(!year) {
    console.error('Please provide a year');
    process.exit(1);
  }

  if(!day) {
    console.error('Please provide a day');
    process.exit(1);
  }

  const input = await fetch(`https://adventofcode.com/${year}/day/${day}/input`, {
    headers: {
      Cookie: `session=${process.env.AOC_SESSION}`,
    },
  })
    .then(r => r.text());

  await Bun.write(`inputs/${year}/${day}.txt`, input!);
}