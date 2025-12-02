import { assertEquals } from "@std/assert";

import { readInput } from "./util.ts";
import { part1, part2 } from "./00.ts";

let input: string;

Deno.test.beforeAll(async () => {
  input = await readInput("00.test");
});

Deno.test("00.1", () => {
  assertEquals(part1(input), "");
});

Deno.test("00.2", () => {
  assertEquals(part2(input), "");
});
