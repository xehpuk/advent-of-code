import { assertEquals } from "@std/assert";

import { readInput } from "./util.ts";
import { part1, part2 } from "./04.ts";

let input: string;

Deno.test.beforeAll(async () => {
  input = await readInput("04.test");
});

Deno.test("04.1", () => {
  assertEquals(part1(input), "13");
});

Deno.test("04.2", () => {
  assertEquals(part2(input), "43");
});
