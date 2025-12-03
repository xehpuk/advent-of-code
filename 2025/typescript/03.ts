import { lines } from "./util.ts";

export function part1(input: string): string {
  return lines(input).map(calcJoltage).reduce(
    (totalJoltage, joltage) => totalJoltage + joltage,
    0,
  ).toString();
}

function calcJoltage(bank: string): number {
  let mostSignificantMax = "";
  let index = -1;
  for (let i = 0; i < bank.length - 1; i++) {
    const battery = bank[i];
    if (battery > mostSignificantMax) {
      mostSignificantMax = battery;
      index = i;
      if (battery === "9") {
        break;
      }
    }
  }
  let leastSignificantMax = "";
  for (let i = index + 1; i < bank.length; i++) {
    const battery = bank[i];
    if (battery > leastSignificantMax) {
      leastSignificantMax = battery;
      if (battery === "9") {
        break;
      }
    }
  }
  return Number.parseInt(mostSignificantMax + leastSignificantMax);
}

export function part2(input: string): string {
  return "// TODO";
}
