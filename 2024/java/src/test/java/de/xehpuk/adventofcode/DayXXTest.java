package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

@Tag("template")
public class DayXXTest {
    @Test
    void part1() {
        assertEquals(0L, Utils.withLines("XX_test", DayXX::part1));
    }

    @Test
    void part2() {
        assertEquals(0L, Utils.withLines("XX_test", DayXX::part2));
    }
}
