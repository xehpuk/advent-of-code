package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day15Test {
    @Test
    void part1_1() {
        assertEquals(10092L, Utils.withLines("15_1_test", Day15::part1));
    }

    @Test
    void part1_2() {
        assertEquals(2028L, Utils.withLines("15_2_test", Day15::part1));
    }

    @Tag("template")
    @Test
    void part2() {
        assertEquals(0L, Utils.withLines("15_test", Day15::part2));
    }
}
