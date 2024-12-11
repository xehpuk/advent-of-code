package de.xehpuk.adventofcode;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Day11 {
    public static long part1(final Stream<String> lines) {
        return solve(lines, 25);
    }

    public static long part2(final Stream<String> lines) {
        return solve(lines, 75);
    }

    static long solve(final Stream<String> lines, final int blinks) {
        var stones = parseLines(lines).stream()
                .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));
        for (int i = 0; i < blinks; i++) {
            final var current = new HashMap<Long, Long>();
            for (final var entry : stones.entrySet()) {
                final long stone = entry.getKey();
                final Long count = entry.getValue();
                final Consumer<Long> merge = key -> current.merge(key, count, Long::sum);

                if (stone == 0L) {
                    merge.accept(1L);
                } else {
                    final int digits = Utils.countDigits(stone);
                    if ((digits & 1) == 0) {
                        final long splitter = Utils.pow(10, digits / 2);
                        merge.accept(stone % splitter);
                        merge.accept(stone / splitter);
                    } else {
                        merge.accept(stone * 2024);
                    }
                }
            }
            stones = current;
        }
        return stones.values().stream()
                .mapToLong(Long::longValue)
                .sum();
    }

    private static List<Long> parseLines(final Stream<String> lines) {
        return Arrays.stream(lines.findAny().get().split(" "))
                .map(Long::parseLong)
                .toList();
    }
}
