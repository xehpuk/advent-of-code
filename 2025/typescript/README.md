# Advent of Code 2025

[Advent of Code 2025 website](https://adventofcode.com/2025)

## Prerequisites

### Runtime

Install [Deno](https://deno.com/).

### Inputs

The inputs are not part of this repository to respect
[the wishes of the author](https://adventofcode.com/2025/about#faq_copying).
Create a directory named `./input` and put them there. For e.g. day 1, the files
should be named like this:

- `01.test.txt` for the test input from the puzzle text
- `01.txt` for the real
  [downloadable puzzle input](https://adventofcode.com/2025/day/1/input)

## Commands

### Running the tests

```sh
deno task test
```

### Solving with the puzzle input

```sh
deno task day <2-digit day>
```

e.g. solving day 1:

```sh
deno task day 01
```

outputs:

```text
01.1: <solution for day 1, part 1>
01.2: <solution for day 1, part 2>
```

### Templates

The files `00.ts` and `00.test.ts` are used as a scaffold to start every day.
