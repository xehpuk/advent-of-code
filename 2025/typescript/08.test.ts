import { assertEquals } from "@std/assert";

import { readInput } from "./util.ts";
import { part1, part2 } from "./08.ts";

let input: string;

Deno.test.beforeAll(async () => {
  input = await readInput("08.test");
});

Deno.test("08.1", () => {
  assertEquals(part1(input, 10), "40");
});

Deno.test("08.2", () => {
  assertEquals(part2(input), "");
});
