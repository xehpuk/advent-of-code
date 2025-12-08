import { sum, toGrid, toLines } from "./util.ts";

export function part1(input: string): string {
  const tachyonManifold = toGrid(input);
  let splitCount = 0;
  for (let r = 0; r < tachyonManifold.length - 1; r++) {
    const row = tachyonManifold[r];
    const nextRow = tachyonManifold[r + 1];
    for (let c = 0; c < row.length; c++) {
      const cell = row[c];
      if (cell === "S") {
        if (nextRow[c] === "^") {
          splitCount++;
          nextRow[c - 1] = "S";
          nextRow[c + 1] = "S";
        } else {
          nextRow[c] = "S";
        }
      }
    }
  }
  return splitCount.toString();
}

export function part2(input: string): string {
  const tachyonManifold: number[][] = parseTachyonManifold(input);
  for (let r = 0; r < tachyonManifold.length - 1; r++) {
    const row = tachyonManifold[r];
    const nextRow = tachyonManifold[r + 1];
    for (let c = 0; c < row.length; c++) {
      const cell = row[c];
      if (cell > 0) {
        if (nextRow[c] === -1) {
          nextRow[c - 1] += cell;
          nextRow[c + 1] += cell;
        } else {
          nextRow[c] += cell;
        }
      }
    }
  }
  return sum(tachyonManifold.at(-1)!).toString();
}

function parseTachyonManifold(input: string): number[][] {
  return toLines(input).map((line) =>
    line.split("").map((cell) => {
      switch (cell) {
        case ".":
          return 0;
        case "S":
          return 1;
        case "^":
          return -1;
        default:
          throw new RangeError(cell);
      }
    })
  );
}
