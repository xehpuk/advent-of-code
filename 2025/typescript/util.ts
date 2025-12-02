export function readInput(day: string) {
  return Deno.readTextFile(`./input/${day}.txt`);
}

/**
 * Splits a string into lines. Skips a potentially empty trailing line.
 */
export function lines(input: string): string[] {
  const lines = input.split(/\r?\n/);
  return lines.at(-1) ? lines : lines.slice(0, -1);
}
