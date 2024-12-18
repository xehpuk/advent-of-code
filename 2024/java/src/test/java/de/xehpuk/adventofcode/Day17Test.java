package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day17Test {
    @Test
    void part1() {
        assertEquals("4,6,3,5,6,3,5,2,1,0", Utils.withLines("17_test", Day17::part1));
    }

    @Tag("template")
    @Test
    void part2() {
        assertEquals(0L, Utils.withLines("17_test", Day17::part2));
    }
}
