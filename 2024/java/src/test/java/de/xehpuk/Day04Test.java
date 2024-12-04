package de.xehpuk;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day04Test {
    @Test
    void part1() {
        assertEquals(18L, Utils.withLines("04_test", Day04::part1));
    }

    @Test
    void part2() {
        assertEquals(9L, Utils.withLines("04_test", Day04::part2));
    }
}
