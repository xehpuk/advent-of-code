package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day19Test {
    @Test
    void part1() {
        assertEquals(6L, Utils.withLines("19_test", Day19::part1));
    }

    @Tag("template")
    @Test
    void part2() {
        assertEquals(0L, Utils.withLines("XX_test", DayXX::part2));
    }
}
