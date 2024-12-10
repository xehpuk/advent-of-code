package de.xehpuk.adventofcode;

import de.xehpuk.adventofcode.Utils.Direction;
import de.xehpuk.adventofcode.Utils.II;

import java.util.*;
import java.util.stream.Collector;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Day10 {
    public static long part1(final Stream<String> lines) {
        return solve(lines, Collectors.toSet());
    }

    public static long part2(final Stream<String> lines) {
        return solve(lines, Collectors.toList());
    }

    private static <C extends Collection<II>> long solve(final Stream<String> lines, final Collector<II, ?, C> collector) {
        final var grid = Utils.parseGrid(lines);
        final Map<Character, Set<II>> map = Utils.mapByR(grid.elements(), HashSet::new);

        long sum = 0;
        for (final var trailhead : map.get('0')) {
            Collection<II> reachablePositions = List.of(trailhead);
            for (int i = 1; i < 10; i++) {
                final char height = (char) ('0' + i);
                final var nextPositions = map.get(height);
                reachablePositions = reachablePositions.stream()
                        .flatMap(reachablePosition -> Arrays.stream(Direction.values())
                                .<II>mapMulti((direction, consumer) -> {
                                    final var nextPosition = direction.move(reachablePosition);
                                    if (nextPositions.contains(nextPosition)) {
                                        consumer.accept(nextPosition);
                                    }
                                }))
                        .collect(collector);
            }
            sum += reachablePositions.size();
        }
        return sum;
    }
}
