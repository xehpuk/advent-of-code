package de.xehpuk;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day05Test {
    @Test
    void part1() {
        assertEquals(143L, Utils.withLines("05_test", Day05::part1));
    }

    @Test
    void part2() {
        assertEquals(123L, Utils.withLines("05_test", Day05::part2));
    }
}
