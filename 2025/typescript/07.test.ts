import { assertEquals } from "@std/assert";

import { readInput } from "./util.ts";
import { part1, part2 } from "./07.ts";

let input: string;

Deno.test.beforeAll(async () => {
  input = await readInput("07.test");
});

Deno.test("07.1", () => {
  assertEquals(part1(input), "21");
});

Deno.test("07.2", () => {
  assertEquals(part2(input), "40");
});
