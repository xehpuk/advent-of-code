export function readInput(day: string): Promise<string> {
  return Deno.readTextFile(`./input/${day}.txt`);
}

/**
 * Splits a string into lines. Skips a potentially empty trailing line.
 */
export function toLines(input: string): string[] {
  const lines = input.split(/\r?\n/);
  return lines.at(-1) ? lines : lines.slice(0, -1);
}

export function toGrid(input: string): string[][] {
  return toLines(input).map((row) => row.split(""));
}

export function fromGrid(grid: string[][]): string {
  return grid.map((row) => row.join("")).join("\n");
}

export function mod(n: number, m: number): number {
  return ((n % m) + m) % m;
}

export function countDigits(n: number): number {
  return n === 0 ? 1 : Math.trunc(Math.log10(n) + 1);
}

export function sum(numbers: number[], initialValue: number = 0): number {
  return numbers.reduce((result, n) => result + n, initialValue);
}

export function product(numbers: number[], initialValue: number = 1): number {
  return numbers.reduce((result, n) => result * n, initialValue);
}

/** See java.util.Arrays.binarySearch0 */
export function binarySearch<T>(
  array: T[],
  key: T,
  comparator: (element: T, element2: T) => number = compare,
  fromIndex: number = 0,
  toIndex: number = array.length,
) {
  let low: number = fromIndex;
  let high: number = toIndex - 1;

  while (low <= high) {
    const mid: number = (low + high) >>> 1;
    const midVal: T = array[mid];
    const cmp = comparator(midVal, key);

    if (cmp < 0) {
      low = mid + 1;
    } else if (cmp > 0) {
      high = mid - 1;
    } else {
      return mid; // key found
    }
  }
  return -(low + 1); // key not found.
}

function compare<T>(element: T, element2: T): number {
  if (element < element2) {
    return -1;
  }
  if (element > element2) {
    return 1;
  }
  return 0;
}

export function insertSorted<T>(
  array: T[],
  key: T,
  comparator: (element: T, element2: T) => number = compare,
  maxLength: number = Infinity,
) {
  const index = binarySearch(array, key, comparator);
  const insertionIndex = index < 0 ? -index - 1 : index + 1;
  if (insertionIndex >= maxLength) {
    return -1;
  }
  array.splice(insertionIndex, 0, key);
  if (array.length > maxLength) {
    array.length = maxLength;
  }
  return insertionIndex;
}
