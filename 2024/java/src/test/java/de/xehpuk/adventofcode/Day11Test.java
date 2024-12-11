package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import java.util.function.ToLongFunction;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day11Test {
    @Test
    void part1() {
        assertEquals(55312L, Utils.withLines("11_1_test", Day11::part1));
    }

    @Test
    void part1_2() {
        assertEquals(7L, Utils.withLines("11_2_test", (ToLongFunction<Stream<String>>) lines -> Day11.solve(lines, 1)));
    }

    @Test
    void part1_3() {
        assertEquals(22L, Utils.withLines("11_1_test", (ToLongFunction<Stream<String>>) lines -> Day11.solve(lines, 6)));
    }
}
