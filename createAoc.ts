import { mkdir } from "fs/promises";
import { appendFile, rm } from "fs/promises";

export async function createAoC(year: string) {
  if(!year) {
    console.error('Please provide a year');
    process.exit(1);
  }

  if(!await Bun.file(`aoc${year}/Cargo.toml`).exists()) {
    console.log("Creating package");
    await Bun.$`cargo new aoc${year}`.quiet();

    await rm(`aoc${year}/.git`, { recursive: true, force: true });

    console.log("Adding aoc dependency");
    await appendFile(`aoc${year}/Cargo.toml`, `aoc = { path = "../aoc" }\n`);

    await rm(`aoc${year}/src/main.rs`);

    await mkdir(`aoc${year}/inputs`);
    await Bun.write(`aoc${year}/inputs/.gitkeep`, "");
    await appendFile(`aoc${year}/.gitignore`, "/inputs/*.txt\n");
  }

  const templateFiles = [
    "README.md",
    "src/days/mod.rs",
    "src/lib.rs",
    "src/main.rs",
  ];

  for(let file of templateFiles) {
    if(await Bun.file(`aoc${year}/${file}`).exists()) {
      console.log(`${file} already exists`);
    } else {
      console.log(`Creating ${file}`);

      Bun.write(`aoc${year}/${file}`, (await Bun.file(`template/${file}`).text()).replace(/{{year}}/g, year));
    }
  }

  const DAY_SOLVER = `use aoc::{DayResult, DaySolver};

  pub struct Day;

  impl DaySolver for Day {
    fn one(&self, _: &str) -> DayResult {
      DayResult::Todo
    }

    fn two(&self, _: &str) -> DayResult {
      DayResult::Todo
    }
  }`;

  for(let day = 1; day <= 25; day++) {
    if(!await Bun.file(`aoc${year}/src/days/day${day}.rs`).exists()) {
      console.log(`Creating day${day}`);
      Bun.write(`aoc${year}/src/days/day${day}.rs`, DAY_SOLVER);
    } else {
      console.log(`day${day} already exists`);
    }
  }
}