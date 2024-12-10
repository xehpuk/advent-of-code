package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day10Test {
    @Test
    void part1_1() {
        assertEquals(1L, Utils.withLines("10_1_test", Day10::part1));
    }

    @Test
    void part1_2() {
        assertEquals(2L, Utils.withLines("10_2_test", Day10::part1));
    }

    @Test
    void part1_3() {
        assertEquals(4L, Utils.withLines("10_3_test", Day10::part1));
    }

    @Test
    void part1_4() {
        assertEquals(3L, Utils.withLines("10_4_test", Day10::part1));
    }

    @Test
    void part1_5() {
        assertEquals(36L, Utils.withLines("10_5_test", Day10::part1));
    }

    @Test
    void part2_1() {
        assertEquals(0L, Utils.withLines("10_1_test", Day10::part2));
    }

    @Test
    void part2_2() {
        assertEquals(0L, Utils.withLines("10_2_test", Day10::part2));
    }

    @Test
    void part2_3() {
        assertEquals(0L, Utils.withLines("10_3_test", Day10::part2));
    }

    @Test
    void part2_4() {
        assertEquals(0L, Utils.withLines("10_4_test", Day10::part2));
    }

    @Test
    void part2_5() {
        assertEquals(0L, Utils.withLines("10_5_test", Day10::part2));
    }
}
