import { readInput } from "./util.ts";

if (import.meta.main) {
  const [day] = Deno.args;
  if (!/^\d\d$/.test(day)) {
    console.error("Expected first argument to be a two-digit day:", day);
    Deno.exit(1);
  }
  const input = await readInput(day);
  const solver = await import(`./${day}.ts`);
  const part1 = solver.part1(input);
  console.log(`${day}.1:`, part1);
  const part2 = solver.part2(input);
  console.log(`${day}.2:`, part2);
}
