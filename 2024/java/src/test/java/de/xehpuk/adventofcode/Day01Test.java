package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

public class Day01Test {
    @Test
    void part1() {
        assertEquals(11L, Utils.withLines("01_test", Day01::part1));
    }

    @Test
    void part2() {
        assertEquals(31L, Utils.withLines("01_test", Day01::part2));
    }
}
