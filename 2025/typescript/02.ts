import { countDigits } from "./util.ts";

export function part1(input: string): string {
  const ranges = parseRanges(input);
  let sumOfInvalidIds = 0;
  for (const range of ranges) {
    for (let id = range.firstId; id <= range.secondId; id++) {
      if (isInvalid(id)) {
        sumOfInvalidIds += id;
      }
    }
  }
  return sumOfInvalidIds.toString();
}

export function part2(input: string): string {
  return "// TODO";
}

function isInvalid(id: number): boolean {
  const digits = countDigits(id);
  if (digits % 2 !== 0) {
    return false;
  }
  const magnitude = 10 ** (digits / 2);
  return Math.trunc(id / magnitude) === id % magnitude;
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
