import { toLines } from "./util.ts";

export function part1(input: string): string {
  const problems = toLines(input).map((line) => line.trim().split(/\s+/));
  const ops = problems.at(-1)!;
  const results = problems[0].map(Number);
  for (let i = 1; i < problems.length - 1; i++) {
    const row = problems[i];
    for (let j = 0; j < row.length; j++) {
      results[j] = ops[j] === "+"
        ? results[j] + Number(row[j])
        : results[j] * Number(row[j]);
    }
  }
  return results.reduce((grandTotal, result) => grandTotal + result, 0)
    .toString();
}

export function part2(input: string): string {
  const lines = toLines(input);
  let grandTotal = 0;
  const numbers = [];
  for (let c = lines[0].length - 1; c >= 0; c--) {
    let numberString = "";
    for (let r = 0; r < lines.length - 1; r++) {
      numberString += lines[r][c];
    }
    numbers.push(Number(numberString));
    const op = lines.at(-1)![c];
    if (op !== " ") {
      grandTotal += numbers.reduce((result, n) =>
        op === "+" ? result + n : result * n
      );
      numbers.length = 0;
      c--;
    }
  }
  return grandTotal.toString();
}
