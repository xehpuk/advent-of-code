package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day11Test {
    @Test
    void part1() {
        assertEquals(0L, Utils.withLines("11_test", Day11::part1));
    }

    @Test
    void part2() {
        assertEquals(0L, Utils.withLines("11_test", Day11::part2));
    }
}
