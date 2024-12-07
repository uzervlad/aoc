# aoc2024

## Notes

Day 1 has an [alternative version](https://github.com/uzervlad/aoc/blob/2024-1-nerdge/aoc2024/src/days/day1.rs) with optimizations that cause cancer. Approach with caution.

<br>

Day 3 is very weird. When I tried benchmarking it, it turned out that if I were to remove the unnecessary parsing for part 1, the solution became over 10x slower.

The solution itself is very ugly since it's all just copy-pasted `.peek()` chaining.

Old solution using regular expressions is available [here](https://github.com/uzervlad/aoc/commit/efd731362f8d473096a7b43944fe17ad26f65374)

<br>

Day 7 uses a recursive solution only in part 1, since benchmarks showed a huge regression in performance in part 2.