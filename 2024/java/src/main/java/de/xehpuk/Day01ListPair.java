package de.xehpuk;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/// An alternative to [Day01] using `(List<Integer>, List<Integer>)` instead of `List<(int, int)>`.
/// Too verbose and probably (can't bother to benchmark) not as efficient.
@SuppressWarnings("unused")
public class Day01ListPair {
    public static long part1(final Stream<String> lines) {
        final var idListPair = parseLines(lines);
        final var sortedLeft = idListPair.left().stream()
                .sorted()
                .toList();
        final var sortedRight = idListPair.right().stream()
                .sorted()
                .toList();

        var sum = 0;
        for (int i = 0; i < sortedLeft.size(); i++) {
            sum += Math.abs(sortedLeft.get(i) - sortedRight.get(i));
        }
        return sum;
    }

    public static long part2(final Stream<String> lines) {
        final var idListPair = parseLines(lines);
        final var rightIdCount = idListPair.right().stream()
                .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));

        return idListPair.left().stream()
                .mapToLong(leftId -> leftId * rightIdCount.getOrDefault(leftId, 0L))
                .sum();
    }

    private static IdListPair parseLines(final Stream<String> lines) {
        return lines
                .map(Day01ListPair::parseLine)
                .collect(
                        () -> new IdListPair(new ArrayList<>(), new ArrayList<>()),
                        (idListPair, idPair) -> {
                            idListPair.left().add(idPair.left());
                            idListPair.right().add(idPair.right());
                        },
                        (idListPair, idListPair2) -> {
                            idListPair.left().addAll(idListPair2.left());
                            idListPair.right().addAll(idListPair2.right());
                        }
                );
    }

    private static IdPair parseLine(final String line) {
        final String[] split = line.split(" {3}", 2);
        return new IdPair(Integer.parseInt(split[0]), Integer.parseInt(split[1]));
    }

    record IdListPair(List<Integer> left, List<Integer> right) {
    }

    record IdPair(int left, int right) {
    }
}
