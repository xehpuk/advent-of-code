package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day03Test {
    @Test
    void part1() {
        assertEquals(161L, Utils.withLines("03_1_test", Day03::part1));
    }

    @Test
    void part2() {
        assertEquals(48L, Utils.withLines("03_2_test", Day03::part2));
    }
}
