import { toLines } from "./util.ts";

export function part1(input: string): string {
  const points = parseTiles(input);
  const rectangles = calcRectangles(points);
  return rectangles.reduce((max, rectangle) => Math.max(max, rectangle.area), 0)
    .toString();
}

/**
 * @see {@link https://github.com/jackysee/aoc/blob/4101ab4/2025/day9.js|jackysee/aoc}
 */
export function part2(input: string): string {
  const points: Point[] = parseTiles(input);
  const rectangles: Rectangle[] = calcRectangles(points).sort((a, b) =>
    b.area - a.area
  );
  const lines: Line[] = calcLines(points);
  return rectangles.find((rectangle) => !intersectLines(lines, rectangle))!
    .area.toString();
}

function parseTiles(input: string): Point[] {
  return toLines(input).map((line) => {
    const [x, y] = line.match(/\d+/g)!.map(Number);
    return { x, y };
  });
}

function calcRectangles(points: Point[]): Rectangle[] {
  const rectangles: Rectangle[] = [];
  for (let i = 0; i < points.length - 1; i++) {
    const point = points[i];
    for (let j = i + 1; j < points.length; j++) {
      const point2 = points[j];
      rectangles.push({
        point,
        point2,
        area: calcArea(point, point2),
      });
    }
  }
  return rectangles;
}

function calcArea(p1: Point, p2: Point): number {
  return (Math.abs(p1.x - p2.x) + 1) * (Math.abs(p1.y - p2.y) + 1);
}

function calcLines(points: Point[]): Line[] {
  const lines: Line[] = [];
  let point = points.at(-1)!;
  for (const point2 of points) {
    lines.push({
      point,
      point2,
    });
    point = point2;
  }
  return lines;
}

function intersectLines(lines: Line[], rect: Rectangle): boolean {
  return lines.some((line) =>
    inRange(line.point.y, line.point2.y, rect.point.y, rect.point2.y) &&
    inRange(line.point.x, line.point2.x, rect.point.x, rect.point2.x)
  );
}

function inRange(a1: number, a2: number, b1: number, b2: number): boolean {
  return !(a1 <= b1 && a1 <= b2 && a2 <= b1 && a2 <= b2) &&
    !(a1 >= b1 && a1 >= b2 && a2 >= b1 && a2 >= b2);
}

interface Line {
  point: Point;
  point2: Point;
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
