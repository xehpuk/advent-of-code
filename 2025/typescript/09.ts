import { toLines } from "./util.ts";

export function part1(input: string): string {
  const tiles = parseTiles(input);
  const rectangles = calcRectangles(tiles);
  return rectangles.reduce((max, rectangle) => Math.max(max, rectangle.area), 0)
    .toString();
}

// FIXME: too slow
export function part2(input: string): string {
  const tiles = parseTiles(input);
  const dimension = calcDimension(tiles);
  console.log(dimension);
  const lines = calcLines(tiles);
  console.log(lines);
  const rectangles = calcRectangles(tiles);
  rectangles.sort((r1, r2) => r2.area - r1.area);
  for (const rectangle of rectangles) {
    console.log(rectangle);
    if (containsOnlyRedOrGreen(rectangle, lines, dimension)) {
      console.log(rectangle);
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

function calcDimension(points: Point[]): Rectangle {
  let minX = Infinity;
  let maxX = -Infinity;
  let minY = Infinity;
  let maxY = -Infinity;
  for (const point of points) {
    if (point.x < minX) {
      minX = point.x;
    } else if (point.x > maxX) {
      maxX = point.x;
    }
    if (point.y < minY) {
      minY = point.y;
    } else if (point.y > maxY) {
      maxY = point.y;
    }
  }
  const point = { x: minX, y: minY };
  const point2 = { x: maxX, y: maxY };
  return {
    point: { x: minX, y: minY },
    point2: { x: maxX, y: maxY },
    area: calcArea(point, point2),
  };
}

function isRedOrGreen(
  point: Point,
  lines: Lines,
  dimension: Rectangle,
): boolean {
  if (isInRange(point.y, lines.x.get(point.x))) {
    return true;
  }
  if (isInRange(point.x, lines.y.get(point.y))) {
    return true;
  }
  let inside = false;
  const y = point.y + 0.5;
  for (let x = point.x + 1; x <= dimension.point2.x; x++) {
    inside = inside !== isInRange(y, lines.x.get(x));
  }
  return inside;
}

function isInRange(n: number, range: Range | undefined): boolean {
  if (range === undefined) {
    return false;
  }
  return range.start <= n && n <= range.end;
}

function containsOnlyRedOrGreen(
  rectangle: Rectangle,
  lines: Lines,
  dimension: Rectangle,
): boolean {
  for (let x = rectangle.point.x; x <= rectangle.point2.x; x++) {
    if (!isRedOrGreen({ x, y: rectangle.point.y }, lines, dimension)) {
      return false;
    }
    if (!isRedOrGreen({ x, y: rectangle.point2.y }, lines, dimension)) {
      return false;
    }
  }
  for (let y = rectangle.point.y + 1; y < rectangle.point2.y; y++) {
    if (!isRedOrGreen({ x: rectangle.point.x, y }, lines, dimension)) {
      return false;
    }
    if (!isRedOrGreen({ x: rectangle.point2.x, y }, lines, dimension)) {
      return false;
    }
  }
  return true;
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
