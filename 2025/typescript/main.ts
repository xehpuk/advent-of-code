import { readInput } from "./util.ts";

if (import.meta.main) {
  const [day] = Deno.args;
  if (!/^\d\d$/.test(day)) {
    console.error("Expected first argument to be a two-digit day:", day);
    Deno.exit(1);
  }
  const input = await readInput(day);
  const solver = await import(`./${day}.ts`);
  console.time(`${day}.1`);
  const part1 = solver.part1(input);
  console.timeEnd(`${day}.1`);
  console.log(`${day}.1:`, part1);
  console.time(`${day}.2`);
  const part2 = solver.part2(input);
  console.timeEnd(`${day}.2`);
  console.log(`${day}.2:`, part2);
}
