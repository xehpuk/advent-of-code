import { insertSorted, product, toLines } from "./util.ts";

export function part1(input: string, n: number = 1000): string {
  const junctionBoxes: Point3d[] = toLines(input).map((line) => {
    const [x, y, z] = line.match(/\d+/g)!.map(Number);
    return { x, y, z };
  });
  const minDistances: {
    junctionBox: Point3d;
    junctionBox2: Point3d;
    distance: number;
  }[] = [];
  for (let i = 0; i < junctionBoxes.length - 1; i++) {
    const junctionBox = junctionBoxes[i];
    for (let j = i + 1; j < junctionBoxes.length; j++) {
      const junctionBox2 = junctionBoxes[j];
      const distance = euclidianDistance(junctionBox, junctionBox2);
      insertSorted(
        minDistances,
        {
          junctionBox,
          junctionBox2,
          distance,
        },
        (junctionBox, junctionBox2) =>
          junctionBox.distance - junctionBox2.distance,
        n,
      );
    }
  }
  console.log(minDistances);

  const circuits: Set<Point3d>[] = junctionBoxes.map((junctionBox) =>
    new Set([junctionBox])
  );
  for (const { junctionBox, junctionBox2 } of minDistances) {
    const circuitIndex = circuits.findIndex((circuit) =>
      circuit.has(junctionBox)
    );
    const circuitIndex2 = circuits.findIndex((circuit) =>
      circuit.has(junctionBox2)
    );
    if (circuitIndex === circuitIndex2) {
      continue;
    }
    console.log(circuitIndex2, "=>", circuitIndex);
    console.log(
      circuits[circuitIndex2],
      "=>",
      circuits[circuitIndex],
    );
    let minIndex: number;
    let maxIndex: number;
    if (circuitIndex < circuitIndex2) {
      minIndex = circuitIndex;
      maxIndex = circuitIndex2;
    } else {
      minIndex = circuitIndex2;
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
