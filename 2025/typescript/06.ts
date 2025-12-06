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
  const ops = Array.from(lines.at(-1)!.matchAll(/\S/g)!, (m) => m[0])
    .toReversed();
  const transposed = transpose(lines.slice(0, -1)).join("\n").split("\n\n")
    .map((block) => block.split("\n").map(Number));
  let grandTotal = 0;
  for (let i = 0; i < transposed.length; i++) {
    grandTotal += transposed[i].reduce((result, n) =>
      ops[i] === "+" ? result + n : result * n
    );
  }
  return grandTotal.toString();
}

function transpose(lines: string[]): string[] {
  const result = [];
  for (let c = lines[0].length - 1; c >= 0; c--) {
    result.push("");
    for (let r = 0; r < lines.length; r++) {
      const char = lines[r][c];
      if (char !== " ") {
        result[result.length - 1] += char;
      }
    }
  }
  return result;
}
