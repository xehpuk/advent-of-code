import { assertEquals } from "@std/assert";

import { readInput } from "./util.ts";
import { part1, part2 } from "./01.ts";

let input: string;

Deno.test.beforeAll(async () => {
  input = await readInput("01.test");
});

Deno.test("01.1", () => {
  assertEquals(part1(input), "3");
});

Deno.test("01.2", () => {
  assertEquals(part2(input), "");
});
