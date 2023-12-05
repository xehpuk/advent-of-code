package de.xehpuk;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class Day05Test {
    @Test
    void part1() {
        assertEquals(35L, Utils.withLines("05_test", Day05::part1));
    }
}
