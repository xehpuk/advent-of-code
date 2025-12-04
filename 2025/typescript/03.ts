import { toLines } from "./util.ts";

export function part1(input: string): string {
  return solve(input, makeCalcJoltage(2));
}

export function part2(input: string): string {
  return solve(input, makeCalcJoltage(12));
}

function solve(input: string, calcJoltageFn: (bank: string) => number): string {
  return toLines(input).map(calcJoltageFn).reduce(
    (totalJoltage, joltage) => totalJoltage + joltage,
    0,
  ).toString();
}

function makeCalcJoltage(count: number): (bank: string) => number {
  return (bank: string): number => {
    const maxes: number[] = Array(count).fill(0);
    let index = -1;
    for (let i = 0; i < maxes.length; i++) {
      for (let j = index + 1; j < bank.length - maxes.length + i + 1; j++) {
        const battery = Number.parseInt(bank[j]);
        if (battery > maxes[i]) {
          maxes[i] = battery;
          index = j;
        }
      }
    }
    return Number.parseInt(maxes.join(""));
  };
}
