package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day09Test {
    @Test
    void part1() {
        assertEquals(1928L, Utils.withLines("09_test", Day09::part1));
    }

    @Test
    void part2() {
        assertEquals(2858L, Utils.withLines("09_test", Day09::part2));
    }
}
