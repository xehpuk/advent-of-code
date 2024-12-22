package de.xehpuk.adventofcode;

import java.util.*;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Day19 {
    private static final Pattern PATTERN_DELIMITER = Pattern.compile(", ", Pattern.LITERAL);

    public static long part1(final Stream<String> lines) {
        final var towels = parseLines(lines);
        final var patterns = towels.patterns()
                .parallelStream()
                .filter(pattern -> !isPossible(pattern, towels.patterns(), pattern.length() - 1))
                .collect(Collectors.toSet());
        final var maxLength = patterns
                .parallelStream()
                .mapToInt(String::length)
                .max()
                .getAsInt();
        return towels.designs()
                .parallelStream()
                .filter(design -> isPossible(design, patterns, maxLength))
                .count();
    }

    public static long part2(final Stream<String> lines) {
        return 0L;
    }

    private static boolean isPossible(final String design, final Set<String> patterns, final int maxLength) {
        if (design.isEmpty()) {
            return true;
        }
        for (int j = Math.min(design.length(), maxLength); j > 0; j--) {
            if (patterns.contains(design.substring(0, j)) &&
                isPossible(design.substring(j), patterns, maxLength)) {
                return true;
            }
        }
        return false;
    }

    private static Towels parseLines(final Stream<String> lines) {
        final var iterator = lines.iterator();
        final var patterns = PATTERN_DELIMITER
                .splitAsStream(iterator.next())
                .collect(Collectors.toSet());
        iterator.next(); // skip empty
        final List<String> designs = new ArrayList<>();
        iterator.forEachRemaining(designs::add);
        return new Towels(patterns, designs);
    }

    record Towels(
            Set<String> patterns,
            List<String> designs
    ) {
    }
}
