package de.xehpuk.adventofcode;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Stream;

public class Day02 {
    public static long part1(final Stream<String> lines) {
        return lines
                .map(Day02::parseLine)
                .filter(Day02::isSafe)
                .count();
    }

    public static long part2(final Stream<String> lines) {
        return lines
                .map(Day02::parseLine)
                .filter(Day02::isSafe2)
                .count();
    }

    private static boolean isSafe(final List<Integer> levels) {
        return unsafeIndex(levels) < 0;
    }

    private static boolean isSafe2(final List<Integer> levels) {
        final var unsafeIndex = unsafeIndex(levels);
        if (unsafeIndex < 0) {
            return true;
        }
        for (int i = Math.max(0, unsafeIndex - 1); i <= unsafeIndex + 1; i++) {
            final var dampenedLevels = dampen(levels, i);
            if (unsafeIndex(dampenedLevels) < 0) {
                return true;
            }
        }
        return false;
    }

    private static ArrayList<Integer> dampen(final List<Integer> levels, final int unsafeIndex) {
        final var dampenedLevels = new ArrayList<>(levels);
        dampenedLevels.remove(unsafeIndex);
        return dampenedLevels;
    }

    private static int unsafeIndex(final List<Integer> levels) {
        if (levels.size() <= 1) {
            return -1;
        }
        final var increasing = levels.get(0) < levels.get(1);
        for (int i = 0; i < levels.size() - 1; i++) {
            final var diff = Math.abs(levels.get(i) - levels.get(i + 1));
            if (diff < 1 || diff > 3 || increasing == levels.get(i) > levels.get(i + 1)) {
                return i;
            }
        }
        return -1;
    }

    private static List<Integer> parseLine(final String line) {
        return Arrays.stream(line.split(" "))
                .map(Integer::parseInt)
                .toList();
    }
}
