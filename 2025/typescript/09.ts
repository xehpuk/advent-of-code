import { toLines } from "./util.ts";

export function part1(input: string): string {
  const tiles = parseTiles(input);
  const rectangles = calcRectangles(tiles);
  return rectangles.reduce((max, rectangle) => Math.max(max, rectangle.area), 0)
    .toString();
}

export function part2(input: string): string {
  const tiles = parseTiles(input);
  const lines = calcLines(tiles);
  const rectangles = calcRectangles(tiles);
  rectangles.sort((r1, r2) => r2.area - r1.area);
  for (const rectangle of rectangles) {
    if (containsOnlyRedOrGreen(rectangle, lines)) {
      return rectangle.area.toString();
    }
  }
  throw new Error("unreachable");
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
        point: { x: Math.min(tile.x, tile2.x), y: Math.min(tile.y, tile2.y) },
        point2: { x: Math.max(tile.x, tile2.x), y: Math.max(tile.y, tile2.y) },
        area,
      });
    }
  }
  return rectangles;
}

function calcArea(p1: Point, p2: Point): number {
  return (Math.abs(p1.x - p2.x) + 1) * (Math.abs(p1.y - p2.y) + 1);
}

function calcLines(points: Point[]): Lines {
  const lines = {
    x: new Map<number, Range>(),
    y: new Map<number, Range>(),
  };
  let previous = points.at(-1)!;
  for (const point of points) {
    if (previous.x === point.x) {
      const range = previous.y < point.y
        ? { start: previous.y, end: point.y }
        : { start: point.y, end: previous.y };
      lines.x.set(point.x, range);
    } else {
      const range = previous.x < point.x
        ? { start: previous.x, end: point.x }
        : { start: point.x, end: previous.x };
      lines.y.set(point.y, range);
    }
    previous = point;
  }
  return lines;
}

function containsOnlyRedOrGreen(
  rectangle: Rectangle,
  lines: Lines,
): boolean {
  for (let x = rectangle.point.x + 1; x < rectangle.point2.x; x++) {
    if (isVerticalStart({ x, y: rectangle.point.y }, lines)) {
      return false;
    }
    if (isVerticalEnd({ x, y: rectangle.point2.y }, lines)) {
      return false;
    }
  }
  for (let y = rectangle.point.y + 1; y < rectangle.point2.y; y++) {
    if (isHorizontalStart({ x: rectangle.point.x, y }, lines)) {
      return false;
    }
    if (isHorizontalEnd({ x: rectangle.point2.x, y }, lines)) {
      return false;
    }
  }
  return true;
}

function isVerticalStart(
  point: Point,
  lines: Lines,
): boolean {
  const range = lines.x.get(point.x);
  if (!range) {
    return false;
  }
  return point.y >= range.start && point.y < range.end;
}

function isVerticalEnd(
  point: Point,
  lines: Lines,
): boolean {
  const range = lines.x.get(point.x);
  if (!range) {
    return false;
  }
  return point.y > range.start && point.y <= range?.end;
}

function isHorizontalStart(
  point: Point,
  lines: Lines,
): boolean {
  const range = lines.y.get(point.y);
  if (!range) {
    return false;
  }
  return point.x >= range.start && point.x < range.end;
}

function isHorizontalEnd(
  point: Point,
  lines: Lines,
): boolean {
  const range = lines.y.get(point.y);
  if (!range) {
    return false;
  }
  return point.x > range.start && point.x <= range?.end;
}

interface Lines {
  x: Map<number, Range>;
  y: Map<number, Range>;
}

interface Range {
  start: number;
  end: number;
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
