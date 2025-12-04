import { toGrid } from "./util.ts";

export function part1(input: string): string {
  const grid = toGrid(input);
  let accessibleRollsOfPaper = 0;
  for (let r = 0; r < grid.length; r++) {
    for (let c = 0; c < grid[r].length; c++) {
      if (isAccessibleRollOfPaper(grid, c, r)) {
        accessibleRollsOfPaper++;
      }
    }
  }
  return accessibleRollsOfPaper.toString();
}

export function part2(input: string): string {
  const grid = toGrid(input);
  let removed = 0;
  while (true) {
    const newlyRemoved = removeRollsOfPaper(grid);
    if (newlyRemoved === 0) {
      break;
    }
    removed += newlyRemoved;
  }
  return removed.toString();
}

function removeRollsOfPaper(grid: string[][]): number {
  let removed = 0;
  for (let r = 0; r < grid.length; r++) {
    for (let c = 0; c < grid[r].length; c++) {
      if (removeRollOfPaper(grid, c, r)) {
        removed++;
      }
    }
  }
  return removed;
}

function removeRollOfPaper(
  grid: string[][],
  x: number,
  y: number,
): boolean {
  if (!isAccessibleRollOfPaper(grid, x, y)) {
    return false;
  }
  grid[y][x] = "x";
  return true;
}

function isAccessibleRollOfPaper(
  grid: string[][],
  x: number,
  y: number,
): boolean {
  if (!isRollOfPaper(grid, x, y)) {
    return false;
  }
  let rollsOfPaper = 0;
  for (let r = -1; r <= 1; r++) {
    for (let c = -1; c <= 1; c++) {
      if (isRollOfPaper(grid, x + c, y + r)) {
        rollsOfPaper++;
      }
    }
  }
  return rollsOfPaper <= 4;
}

function isRollOfPaper(grid: string[][], x: number, y: number): boolean {
  const row = grid[y];
  return !!row && row[x] === "@";
}
