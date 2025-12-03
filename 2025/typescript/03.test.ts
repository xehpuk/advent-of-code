import { assertEquals } from "@std/assert";

import { readInput } from "./util.ts";
import { part1, part2 } from "./03.ts";

let input: string;

Deno.test.beforeAll(async () => {
  input = await readInput("03.test");
});

Deno.test("03.1", () => {
  assertEquals(part1(input), "357");
});

Deno.test("03.2", () => {
  assertEquals(part2(input), "3121910778619");
});
