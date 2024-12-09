package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day08Test {
    @Test
    void part1() {
        assertEquals(14L, Utils.withLines("08_test", Day08::part1));
    }

    @Test
    void part2() {
        assertEquals(34L, Utils.withLines("08_test", Day08::part2));
    }
}
