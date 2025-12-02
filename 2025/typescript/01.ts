import { lines } from "./util.ts";

export function part1(input: string): string {
  let zeroes = 0;
  let pointer = 50;
  for (const line of lines(input)) {
    const rotation = parseRotation(line);
    const sign = rotation.direction === "L" ? -1 : 1;
    pointer += sign * rotation.distance;
    if (pointer % 100 === 0) {
      zeroes++;
    }
  }
  return zeroes.toString();
}

export function part2(input: string): string {
  return "// TODO";
}

function parseRotation(line: string): Rotation {
  const direction = line[0];
  if (direction !== "L" && direction !== "R") {
    throw new TypeError(`Unknown direction: '${direction}'`);
  }
  const distance = Number.parseInt(line.substring(1));
  return { direction, distance };
}

interface Rotation {
  direction: "L" | "R";
  distance: number;
}
