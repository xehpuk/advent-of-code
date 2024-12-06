package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day06Test {
    @Test
    void part1() {
        assertEquals(41L, Utils.withLines("06_test", Day06::part1));
    }

    @Test
    void part2() {
        assertEquals(6L, Utils.withLines("06_test", Day06::part2));
    }
}
