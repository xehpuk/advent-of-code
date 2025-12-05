import { assertEquals } from "@std/assert";

import { readInput } from "./util.ts";
import { overlap, part1, part2 } from "./05.ts";

let input: string;

Deno.test.beforeAll(async () => {
  input = await readInput("05.test");
});

Deno.test("05.1", () => {
  assertEquals(part1(input), "3");
});

Deno.test("05.2 overlap", () => {
  assertEquals(overlap({ start: 3, end: 5 }, { start: 3, end: 5 }), true);
  assertEquals(overlap({ start: 4, end: 5 }, { start: 3, end: 5 }), true);
  assertEquals(overlap({ start: 5, end: 5 }, { start: 3, end: 5 }), true);
  assertEquals(overlap({ start: 3, end: 5 }, { start: 4, end: 5 }), true);
  assertEquals(overlap({ start: 3, end: 5 }, { start: 5, end: 5 }), true);
  assertEquals(overlap({ start: 3, end: 5 }, { start: 5, end: 6 }), true);
  assertEquals(overlap({ start: 5, end: 6 }, { start: 3, end: 5 }), true);

  assertEquals(overlap({ start: 3, end: 5 }, { start: 6, end: 7 }), false);
  assertEquals(overlap({ start: 6, end: 7 }, { start: 3, end: 5 }), false);
});

Deno.test("05.2", () => {
  assertEquals(part2(input), "14");
});
