package de.xehpuk.adventofcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day12Test {
    @Test
    void part1_1() {
        assertEquals(140L, Utils.withLines("12_1_test", Day12::part1));
    }

    @Test
    void part1_2() {
        assertEquals(772L, Utils.withLines("12_2_test", Day12::part1));
    }

    @Test
    void part1_3() {
        assertEquals(1930L, Utils.withLines("12_3_test", Day12::part1));
    }

    @Test
    void part2_1() {
        assertEquals(80L, Utils.withLines("12_1_test", Day12::part2));
    }

    @Test
    void part2_2() {
        assertEquals(436L, Utils.withLines("12_2_test", Day12::part2));
    }

    @Test
    void part2_3() {
        assertEquals(1206L, Utils.withLines("12_3_test", Day12::part2));
    }

    @Test
    void part2_4() {
        assertEquals(236L, Utils.withLines("12_4_test", Day12::part2));
    }

    @Test
    void part2_5() {
        assertEquals(368L, Utils.withLines("12_5_test", Day12::part2));
    }
}
