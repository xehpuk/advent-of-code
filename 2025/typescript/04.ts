import { lines } from "./util.ts";

export function part1(input: string): string {
  const rows = lines(input);
  let accessibleRollsOfPaper = 0;
  for (let r = 0; r < rows.length; r++) {
    for (let c = 0; c < rows[r].length; c++) {
      if (isAccessibleRollOfPaper(rows, c, r)) {
        accessibleRollsOfPaper++;
      }
    }
  }
  return accessibleRollsOfPaper.toString();
}

export function part2(input: string): string {
  return "// TODO";
}

function isAccessibleRollOfPaper(
  rows: string[],
  x: number,
  y: number,
): boolean {
  if (!isRollOfPaper(rows, x, y)) {
    return false;
  }
  let rollsOfPaper = 0;
  for (let r = -1; r <= 1; r++) {
    for (let c = -1; c <= 1; c++) {
      if (isRollOfPaper(rows, x + c, y + r)) {
        rollsOfPaper++;
      }
    }
  }
  return rollsOfPaper <= 4;
}

function isRollOfPaper(rows: string[], x: number, y: number): boolean {
  const row = rows[y];
  return !!row && row[x] === "@";
}
