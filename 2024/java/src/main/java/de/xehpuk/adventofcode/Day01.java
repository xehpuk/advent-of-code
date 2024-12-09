package de.xehpuk.adventofcode;

import de.xehpuk.adventofcode.Utils.II;

import java.util.Comparator;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Day01 {
    public static long part1(final Stream<String> lines) {
        final var idPairs = lines
                .map(Day01::parseLine)
                .toList();
        final var sortedByLeft = idPairs.stream()
                .sorted(Comparator.comparing(II::l))
                .toList();
        final var sortedByRight = idPairs.stream()
                .sorted(Comparator.comparing(II::r))
                .toList();

        var sum = 0;
        for (int i = 0; i < sortedByLeft.size(); i++) {
            final var leftId = sortedByLeft.get(i).l();
            final var rightId = sortedByRight.get(i).r();

            sum += Math.abs(leftId - rightId);
        }
        return sum;
    }

    public static long part2(final Stream<String> lines) {
        final var idPairs = lines
                .map(Day01::parseLine)
                .toList();
        final var rightIdCount = idPairs.stream()
                .collect(Collectors.groupingBy(II::r, Collectors.counting()));

        return idPairs.stream()
                .mapToInt(II::l)
                .mapToLong(leftId -> leftId * rightIdCount.getOrDefault(leftId, 0L))
                .sum();
    }

    private static II parseLine(final String line) {
        final String[] split = line.split(" {3}", 2);
        return new II(Integer.parseInt(split[0]), Integer.parseInt(split[1]));
    }
}
