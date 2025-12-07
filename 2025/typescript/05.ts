import { sum, toLines } from "./util.ts";

export function part1(input: string): string {
  const ingredients = parseIngredients(input);
  return ingredients.available.filter((ingredient) =>
    ingredients.freshRanges.some((range) =>
      range.start <= ingredient && ingredient <= range.end
    )
  ).length.toString();
}

export function part2(input: string): string {
  const { freshRanges } = parseIngredients(input);
  for (let i = freshRanges.length - 2; i >= 0; i--) {
    for (let j = freshRanges.length - 1; j > i; j--) {
      if (overlap(freshRanges[i], freshRanges[j])) {
        // merge ranges
        freshRanges[i] = {
          start: Math.min(
            freshRanges[i].start,
            freshRanges[j].start,
          ),
          end: Math.max(
            freshRanges[i].end,
            freshRanges[j].end,
          ),
        };
        freshRanges.splice(j, 1);
      }
    }
  }
  return sum(freshRanges.map((range) => range.end - range.start + 1))
    .toString();
}

function parseIngredients(input: string): Ingredients {
  const lines = toLines(input);
  const freshRanges: Range[] = [];
  let i = 0;
  while (lines[i].length > 0) {
    freshRanges.push(parseRange(lines[i]));
    i++;
  }
  const available: number[] = [];
  for (let j = i + 1; j < lines.length; j++) {
    available.push(Number.parseInt(lines[j]));
  }
  return {
    freshRanges,
    available,
  };
}

export function overlap(range1: Range, range2: Range): boolean {
  return range1.start <= range2.end && range2.start <= range1.end;
}

function parseRange(input: string): Range {
  const [, first, second] = input.match(/(\d+)-(\d+)/)!;
  return {
    start: Number.parseInt(first),
    end: Number.parseInt(second),
  };
}

interface Range {
  start: number;
  end: number;
}

interface Ingredients {
  freshRanges: Range[];
  available: number[];
}
