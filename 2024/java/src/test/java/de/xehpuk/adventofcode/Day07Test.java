package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day07Test {
    @Test
    void part1() {
        assertEquals(3749L, Utils.withLines("07_test", Day07::part1));
    }

    @Test
    void part2() {
        assertEquals(11387L, Utils.withLines("07_test", Day07::part2));
    }
}
