package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

public class Day13Test {
    @Test
    void part1() {
        assertEquals(480L, Utils.withLines("13_test", Day13::part1));
    }

    @Test
    void part1_tokens() {
        long[] expected = {280, 0, 200, 0};
        long[] actual = Utils.withLines("13_test", (Stream<String> lines) -> Day13.tokens(lines).toArray());
        assertArrayEquals(expected, actual);
    }

    @Test
    void part1_1() {
        long[] expected = {1, 1, 3, 5, 5, 7, 4, 4, 4, 4, 0, 0, 11, 5, 11, 33};
        long[] actual = Utils.withLines("13_1_test", (Stream<String> lines) -> Day13.tokens(lines).toArray());
        assertArrayEquals(expected, actual, () ->
                "\nExpected: " + Arrays.toString(expected) +
                "\nActual: " + Arrays.toString(actual));
    }

    @Test
    void part2_tokens() {
        long[] actual = Utils.withLines("13_test", (Stream<String> lines) -> Day13.tokens2(lines).toArray());
        assertEquals(0, actual[0], "0");
        assertNotEquals(0, actual[1], "1");
        assertEquals(0, actual[2], "2");
        assertNotEquals(0, actual[3], "3");
    }
}
