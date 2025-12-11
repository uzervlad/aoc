export async function fetchInput(year?: string, day?: string) {
  year ??= new Date().getFullYear().toString();
  day ??= new Date().getDate().toString();

  console.log(`Downloading input for year ${year} day ${day}`);

  const input = await fetch(`https://adventofcode.com/${year}/day/${day}/input`, {
    headers: {
      Cookie: `session=${process.env.AOC_SESSION}`,
    },
  })
    .then(r => r.text());

  await Bun.write(`inputs/${year}/${day}.txt`, input!);
}