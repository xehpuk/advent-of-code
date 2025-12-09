import { assertEquals } from "@std/assert";

import { readInput } from "./util.ts";
import { part1, part2 } from "./09.ts";

let input: string;

Deno.test.beforeAll(async () => {
  input = await readInput("09.test");
});

Deno.test("09.1", () => {
  assertEquals(part1(input), "50");
});

Deno.test("09.2", () => {
  assertEquals(part2(input), "24");
});
