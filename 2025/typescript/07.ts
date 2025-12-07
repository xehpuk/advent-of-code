import { toLines } from "./util.ts";

export function part1(input: string): string {
  const tachyonManifold = toLines(input).map((line) => line.split(""));
  let splitCount = 0;
  for (let r = 0; r < tachyonManifold.length - 1; r++) {
    const row = tachyonManifold[r];
    const nextRow = tachyonManifold[r + 1];
    for (let c = 0; c < row.length; c++) {
      const cell = row[c];
      if (cell === "S" || cell === "|") {
        if (nextRow[c] === "^") {
          splitCount++;
          if (c > 0) {
            nextRow[c - 1] = "|";
          }
          if (c < row.length - 1) {
            nextRow[c + 1] = "|";
          }
        } else {
          nextRow[c] = "|";
        }
      }
    }
  }
  return splitCount.toString();
}

export function part2(input: string): string {
  return "// TODO";
}
