import { product, sum, toLines } from "./util.ts";

export function part1(input: string): string {
  const problems = toLines(input).map((line) => Array.from(line.match(/\S+/g)!));
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
  return sum(results).toString();
}

export function part2(input: string): string {
  const lines = toLines(input);
  let grandTotal = 0;
  const numbers = [];
  for (let c = lines[0].length - 1; c >= 0; c--) {
    let number = 0;
    for (let r = 0; r < lines.length - 1; r++) {
      const digit = lines[r][c];
      if (digit !== " ") {
        number = 10 * number + Number(digit);
      }
    }
    numbers.push(number);
    const op = lines.at(-1)![c];
    if (op !== " ") {
      grandTotal += op === "+" ? sum(numbers) : product(numbers);
      numbers.length = 0;
      c--;
    }
  }
  return grandTotal.toString();
}
