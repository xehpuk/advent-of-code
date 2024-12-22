package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day22Test {
    @Test
    void part1() {
        assertEquals(37327623L, Utils.withLines("22_1_test", Day22::part1));
    }

    @Tag("template")
    @Test
    void part2() {
        assertEquals(23L, Utils.withLines("22_2_test", Day22::part2));
    }
}
