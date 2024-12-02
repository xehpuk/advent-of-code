package de.xehpuk;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day02Test {
    @Test
    void part1() {
        assertEquals(2L, Utils.withLines("02_test", Day02::part1));
    }

    @Test
    void part2() {
        assertEquals(4L, Utils.withLines("02_test", Day02::part2));
    }
}
