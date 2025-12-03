import { countDigits } from "./util.ts";

export function part1(input: string): string {
  return solve(input, isInvalid1);
}

export function part2(input: string): string {
  return solve(input, isInvalid2);
}

function solve(input: string, invalidFn: (id: number) => boolean): string {
  const ranges = parseRanges(input);
  let sumOfInvalidIds = 0;
  for (const range of ranges) {
    for (let id = range.firstId; id <= range.secondId; id++) {
      if (invalidFn(id)) {
        sumOfInvalidIds += id;
      }
    }
  }
  return sumOfInvalidIds.toString();
}

function isInvalid1(id: number): boolean {
  const digits = countDigits(id);
  if (digits % 2 !== 0) {
    return false;
  }
  const magnitude = 10 ** (digits / 2);
  return Math.trunc(id / magnitude) === id % magnitude;
}

function isInvalid2(id: number): boolean {
  return /^(.+)\1+$/.test(id.toString());
}

function parseRanges(input: string): Range[] {
  return Array.from(input.matchAll(/(\d+)-(\d+)/g), ([, first, second]) => ({
    firstId: Number.parseInt(first),
    secondId: Number.parseInt(second),
  }));
}

interface Range {
  firstId: number;
  secondId: number;
}
