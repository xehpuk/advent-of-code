import { insertSorted, product, toLines } from "./util.ts";

export function part1(input: string, n: number = 1000): string {
  const junctionBoxes = parseJunctionBoxes(input);
  const minConnections = calcConnections(
    junctionBoxes,
    (connections, connection) =>
      insertSorted(
        connections,
        connection,
        (connection, connection2) => connection.distance - connection2.distance,
        n,
      ),
  );
  const circuits = initCircuits(junctionBoxes);
  for (const { junctionBox, junctionBox2 } of minConnections) {
    connect(circuits, junctionBox, junctionBox2);
  }
  circuits.sort((circuit, circuit2) => circuit2.size - circuit.size);
  return product(circuits.slice(0, 3).map((circuit) => circuit.size))
    .toString();
}

export function part2(input: string): string {
  const junctionBoxes = parseJunctionBoxes(input);
  const connections = calcConnections(
    junctionBoxes,
    (connections, connection) => connections.push(connection),
  );
  connections.sort((connection, connection2) =>
    connection.distance - connection2.distance
  );
  const circuits = initCircuits(junctionBoxes);
  for (const { junctionBox, junctionBox2 } of connections) {
    connect(circuits, junctionBox, junctionBox2);
    if (circuits.length === 1) {
      return (junctionBox.x * junctionBox2.x).toString();
    }
  }
  throw new Error("unreachable");
}

function parseJunctionBoxes(input: string): Point3d[] {
  return toLines(input).map((line) => {
    const [x, y, z] = line.match(/\d+/g)!.map(Number);
    return { x, y, z };
  });
}

function initCircuits(junctionBoxes: Point3d[]): Set<Point3d>[] {
  return junctionBoxes.map((junctionBox) => new Set([junctionBox]));
}

function calcConnections(
  junctionBoxes: Point3d[],
  insertionFn: (connections: Connection[], connection: Connection) => void,
): Connection[] {
  const connections: Connection[] = [];
  for (let i = 0; i < junctionBoxes.length - 1; i++) {
    const junctionBox = junctionBoxes[i];
    for (let j = i + 1; j < junctionBoxes.length; j++) {
      const junctionBox2 = junctionBoxes[j];
      const distance = euclidianDistance(junctionBox, junctionBox2);
      insertionFn(connections, {
        junctionBox,
        junctionBox2,
        distance,
      });
    }
  }
  return connections;
}

function connect(
  circuits: Set<Point3d>[],
  junctionBox: Point3d,
  junctionBox2: Point3d,
): void {
  const circuitIndex = circuits.findIndex((circuit) =>
    circuit.has(junctionBox)
  );
  const circuitIndex2 = circuits.findIndex((circuit) =>
    circuit.has(junctionBox2)
  );
  if (circuitIndex === circuitIndex2) {
    return;
  }
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

function euclidianDistance(p1: Point3d, p2: Point3d): number {
  return Math.sqrt(
    (p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2 + (p1.z - p2.z) ** 2,
  );
}

interface Connection {
  junctionBox: Point3d;
  junctionBox2: Point3d;
  distance: number;
}

interface Point3d {
  x: number;
  y: number;
  z: number;
}
