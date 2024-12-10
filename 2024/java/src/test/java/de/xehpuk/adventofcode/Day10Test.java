package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day10Test {
    @Test
    void part1_1() {
        assertEquals(1L, Utils.withLines("10_1_1_test", Day10::part1));
    }

    @Test
    void part1_2() {
        assertEquals(2L, Utils.withLines("10_1_2_test", Day10::part1));
    }

    @Test
    void part1_3() {
        assertEquals(4L, Utils.withLines("10_1_3_test", Day10::part1));
    }

    @Test
    void part1_4() {
        assertEquals(3L, Utils.withLines("10_1_4_test", Day10::part1));
    }

    @Test
    void part1_5() {
        assertEquals(36L, Utils.withLines("10_test", Day10::part1));
    }

    @Test
    void part2_1() {
        assertEquals(3L, Utils.withLines("10_2_1_test", Day10::part2));
    }

    @Test
    void part2_2() {
        assertEquals(13L, Utils.withLines("10_2_2_test", Day10::part2));
    }

    @Test
    void part2_3() {
        assertEquals(81L, Utils.withLines("10_test", Day10::part2));
    }
}
