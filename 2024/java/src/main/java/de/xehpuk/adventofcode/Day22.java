package de.xehpuk.adventofcode;

import java.util.stream.LongStream;
import java.util.stream.Stream;

public class Day22 {
    public static long part1(final Stream<String> lines) {
        return parseLines(lines).map(Day22::next2000th).sum();
    }

    public static long part2(final Stream<String> lines) {
        return 0L;
    }

    private static long next2000th(long n) {
        for (int i = 0; i < 2000; i++) {
            n = next(n);
        }
        return n;
    }

    private static long next(long n) {
        n = n ^ n * 64;
        n = n % 16777216;
        n = n ^ n / 32;
        n = n % 16777216;
        n = n ^ n * 2048;
        n = n % 16777216;
        return n;
    }

    private static LongStream parseLines(final Stream<String> lines) {
        return lines.mapToLong(Long::parseLong);
    }
}
