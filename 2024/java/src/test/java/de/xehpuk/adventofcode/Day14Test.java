package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day14Test {
    @Test
    void part1() {
        assertEquals(12L, Utils.withLines("14_test", (Stream<String> value) -> Day14.part1(value, 11, 7)));
    }
}
