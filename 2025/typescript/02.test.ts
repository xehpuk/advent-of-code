import { assertEquals } from "@std/assert";

import { readInput } from "./util.ts";
import { part1, part2 } from "./02.ts";

let input: string;

Deno.test.beforeAll(async () => {
  input = await readInput("02.test");
});

Deno.test("02.1", () => {
  assertEquals(part1(input), "1227775554");
});

Deno.test("02.2", () => {
  assertEquals(part2(input), "");
});
