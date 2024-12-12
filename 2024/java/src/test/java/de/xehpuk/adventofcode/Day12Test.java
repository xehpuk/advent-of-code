package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day12Test {
    @Test
    void part1_1() {
        assertEquals(140L, Utils.withLines("12_1_test", Day12::part1));
    }

    @Test
    void part1_2() {
        assertEquals(772L, Utils.withLines("12_2_test", Day12::part1));
    }

    @Test
    void part1_3() {
        assertEquals(1930L, Utils.withLines("12_3_test", Day12::part1));
    }

    @Disabled
    @Test
    void part2() {
        assertEquals(0L, Utils.withLines("12_test", Day12::part2));
    }
}
