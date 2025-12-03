export function readInput(day: string): Promise<string> {
  return Deno.readTextFile(`./input/${day}.txt`);
}

/**
 * Splits a string into lines. Skips a potentially empty trailing line.
 */
export function lines(input: string): string[] {
  const lines = input.split(/\r?\n/);
  return lines.at(-1) ? lines : lines.slice(0, -1);
}

export function mod(n: number, m: number): number {
  return ((n % m) + m) % m;
}

export function countDigits(n: number): number {
  return n === 0 ? 1 : Math.trunc(Math.log10(n) + 1);
}
