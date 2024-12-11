package de.xehpuk.adventofcode;

import java.util.*;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Day11 {
    public static long part1(final Stream<String> lines) {
        return part1(lines, 25);
    }

    static long part1(final Stream<String> lines, final int blinks) {
        final var stones = parseLines(lines);
        for (int i = 0; i < blinks; i++) {
            for (int j = stones.size() - 1; j >= 0; j--) {
                final long stone = stones.get(j);
                if (stone == 0) {
                    stones.set(j, 1L);
                } else {
                    final int digits = Utils.countDigits(stone);
                    if ((digits & 1) == 0) {
                        final long splitter = Utils.pow(10, digits / 2);
                        stones.set(j, stone % splitter);
                        stones.add(j, stone / splitter);
                    } else {
                        stones.set(j, stone * 2024);
                    }
                }
            }
        }
        return stones.size();
    }

    public static long part2(final Stream<String> lines) {
        return part2(lines, 75);
    }

    static long part2(final Stream<String> lines, final int blinks) {
        var stones = parseLines(lines).stream()
                .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));
        for (int i = 0; i < blinks; i++) {
            final var current = new HashMap<Long, Long>();
            for (final var entry : stones.entrySet()) {
                final long stone = entry.getKey();
                final long count = entry.getValue();
                if (stone == 0L) {
                    current.merge(1L, count, Long::sum);
                } else {
                    final int digits = Utils.countDigits(stone);
                    if ((digits & 1) == 0) {
                        final long splitter = Utils.pow(10, digits / 2);
                        current.merge(stone % splitter, count, Long::sum);
                        current.merge(stone / splitter, count, Long::sum);
                    } else {
                        current.merge(stone * 2024, count, Long::sum);
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
        return new ArrayList<>(Arrays.stream(lines.findAny().get().split(" "))
                .map(Long::parseLong)
                .toList());
    }
}
