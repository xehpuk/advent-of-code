package de.xehpuk.adventofcode;

import de.xehpuk.adventofcode.Utils.II;
import de.xehpuk.adventofcode.Utils.Pair;

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
        final var sortedLeft = idListPair.l().stream()
                .sorted()
                .toList();
        final var sortedRight = idListPair.r().stream()
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
        final var rightIdCount = idListPair.r().stream()
                .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));

        return idListPair.l().stream()
                .mapToLong(leftId -> leftId * rightIdCount.getOrDefault(leftId, 0L))
                .sum();
    }

    private static Pair<List<Integer>, List<Integer>> parseLines(final Stream<String> lines) {
        return lines
                .map(Day01ListPair::parseLine)
                .collect(
                        () -> new Pair<>(new ArrayList<>(), new ArrayList<>()),
                        (idListPair, idPair) -> {
                            idListPair.l().add(idPair.l());
                            idListPair.r().add(idPair.r());
                        },
                        (idListPair, idListPair2) -> {
                            idListPair.l().addAll(idListPair2.l());
                            idListPair.r().addAll(idListPair2.r());
                        }
                );
    }

    private static II parseLine(final String line) {
        final String[] split = line.split(" {3}", 2);
        return new II(Integer.parseInt(split[0]), Integer.parseInt(split[1]));
    }
}
