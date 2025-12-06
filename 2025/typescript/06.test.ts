import { assertEquals } from "@std/assert";

import { readInput } from "./util.ts";
import { part1, part2 } from "./06.ts";

let input: string;

Deno.test.beforeAll(async () => {
  input = await readInput("06.test");
});

Deno.test("06.1", () => {
  assertEquals(part1(input), "4277556");
});

Deno.test("06.2", () => {
  assertEquals(part2(input), "3263827");
});
