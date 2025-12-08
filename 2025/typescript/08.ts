import { product, toLines } from "./util.ts";

// FIXME this connects all junction boxes with their nearest neighbor once
export function part1(input: string): string {
  const junctionBoxes: Point3d[] = toLines(input).map((line) => {
    const [x, y, z] = line.match(/\d+/g)!.map(Number);
    return { x, y, z };
  });
  const circuits: Set<Point3d>[] = junctionBoxes.map((junctionBox) =>
    new Set([junctionBox])
  );
  for (const junctionBox of junctionBoxes) {
    let minDistance = Infinity;
    let nearestNeighbor = junctionBox;
    for (const junctionBox2 of junctionBoxes) {
      if (junctionBox === junctionBox2) {
        continue;
      }
      const distance = euclidianDistance(junctionBox, junctionBox2);
      if (distance < minDistance) {
        minDistance = distance;
        nearestNeighbor = junctionBox2;
      }
    }
    console.log(junctionBox, nearestNeighbor, minDistance);
    const circuitIndex = circuits.findIndex((circuit) =>
      circuit.has(junctionBox)
    );
    const circuitIndexNearestNeighbor = circuits.findIndex((circuit) =>
      circuit.has(nearestNeighbor)
    );
    if (circuitIndex === circuitIndexNearestNeighbor) {
      continue;
    }
    console.log(circuitIndexNearestNeighbor, "=>", circuitIndex);
    console.log(
      circuits[circuitIndexNearestNeighbor],
      "=>",
      circuits[circuitIndex],
    );
    let minIndex: number;
    let maxIndex: number;
    if (circuitIndex < circuitIndexNearestNeighbor) {
      minIndex = circuitIndex;
      maxIndex = circuitIndexNearestNeighbor;
    } else {
      minIndex = circuitIndexNearestNeighbor;
      maxIndex = circuitIndex;
    }
    circuits[minIndex] = circuits[minIndex].union(
      circuits.splice(maxIndex, 1)[0],
    );
  }
  circuits.sort((circuit1, circuit2) => circuit2.size - circuit1.size);
  console.log(circuits);
  return product(circuits.map((circuit) => circuit.size).slice(0, 3))
    .toString();
}

export function part2(input: string): string {
  return "// TODO";
}

function euclidianDistance(p1: Point3d, p2: Point3d): number {
  return Math.sqrt(
    (p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2 + (p1.z - p2.z) ** 2,
  );
}

interface Point3d {
  x: number;
  y: number;
  z: number;
}
