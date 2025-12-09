import { toLines } from "./util.ts";

export function part1(input: string): string {
  const tiles = parseTiles(input);
  const rectangles = calcRectangles(tiles);
  return rectangles.reduce((max, rectangle) => Math.max(max, rectangle.area), 0)
    .toString();
}

export function part2(input: string): string {
  return "// TODO";
}

function parseTiles(input: string): Point[] {
  return toLines(input).map((line) => {
    const [x, y] = line.match(/\d+/g)!.map(Number);
    return { x, y };
  });
}

function calcRectangles(tiles: Point[]): Rectangle[] {
  const rectangles: Rectangle[] = [];
  for (let i = 0; i < tiles.length - 1; i++) {
    const tile = tiles[i];
    for (let j = i + 1; j < tiles.length; j++) {
      const tile2 = tiles[j];
      const area = calcArea(tile, tile2);
      rectangles.push({
        point: tile,
        point2: tile2,
        area,
      });
    }
  }
  return rectangles;
}

function calcArea(p1: Point, p2: Point): number {
  return (Math.abs(p1.x - p2.x) + 1) * (Math.abs(p1.y - p2.y) + 1);
}

interface Rectangle {
  point: Point;
  point2: Point;
  area: number;
}

interface Point {
  x: number;
  y: number;
}
