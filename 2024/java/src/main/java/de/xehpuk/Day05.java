package de.xehpuk;

import java.util.*;
import java.util.function.Function;
import java.util.function.Predicate;
import java.util.stream.Gatherer;
import java.util.stream.Stream;

import static java.util.function.Predicate.not;

public class Day05 {
    public static long part1(final Stream<String> lines) {
        final var orderedUpdates = parseLines(lines);

        return orderedUpdates.updates().stream()
                .filter(correctlyOrdered(orderedUpdates.ordering()))
                .mapToInt(Utils::middleElement)
                .sum();
    }

    public static long part2(final Stream<String> lines) {
        final var orderedUpdates = parseLines(lines);

        return orderedUpdates.updates().stream()
                .filter(not(correctlyOrdered(orderedUpdates.ordering())))
                .map(sortedUpdate(orderedUpdates.ordering()))
                .mapToInt(Utils::middleElement)
                .sum();
    }

    private static Function<List<Integer>, List<Integer>> sortedUpdate(final Map<Integer, Set<Integer>> ordering) {
        final var comparator = sortedComparator(ordering);
        return update -> update.stream()
                .sorted(comparator).toList();
    }

    private static Comparator<Integer> sortedComparator(final Map<Integer, Set<Integer>> ordering) {
        return (left, right) -> {
            final var prevLeft = ordering.get(left);
            if (prevLeft == null) {
                final var prevRight = ordering.get(right);
                return prevRight == null || !prevRight.contains(left)
                        ? 0
                        : 1;
            }
            return prevLeft.contains(right)
                    ? -1
                    : 0;
        };
    }

    private static OrderedUpdates parseLines(final Stream<String> lines) {
        return lines
                .gather(orderedUpdates())
                .findAny().get();
    }

    private static Predicate<List<Integer>> correctlyOrdered(final Map<Integer, Set<Integer>> ordering) {
        return update -> {
            for (int i = 0; i < update.size(); i++) {
                final var page = update.get(i);
                for (int j = 0; j < i; j++) {
                    final var prevPage = update.get(j);
                    if (ordering.getOrDefault(page, Set.of()).contains(prevPage)) {
                        return false;
                    }
                }
            }
            return true;
        };
    }

    private static Gatherer<String, ?, OrderedUpdates> orderedUpdates() {
        class State {
            boolean parsingUpdates;
            final OrderedUpdates orderedUpdates = new OrderedUpdates(new HashMap<>(), new ArrayList<>());
        }

        return Gatherer.ofSequential(
                State::new,
                Gatherer.Integrator.ofGreedy((state, element, _) -> {
                    if (state.parsingUpdates) {
                        state.orderedUpdates.updates().add(parseUpdates(element));
                    } else if (element.isEmpty()) {
                        state.parsingUpdates = true;
                    } else {
                        final var ordering = parseOrdering(element);
                        state.orderedUpdates.ordering().compute(ordering.getFirst(), (_, succPages) -> {
                            if (succPages == null) {
                                succPages = new HashSet<>();
                            }
                            succPages.add(ordering.getLast());
                            return succPages;
                        });
                    }
                    return true;
                }),
                (state, downstream) -> downstream.push(state.orderedUpdates)
        );
    }

    private static List<Integer> parseUpdates(final String line) {
        return Arrays.stream(line.split(","))
                .map(Integer::parseInt)
                .toList();
    }

    private static List<Integer> parseOrdering(String element) {
        return Arrays.stream(element.split("\\|", 2))
                .map(Integer::parseInt)
                .toList();
    }

    record OrderedUpdates(Map<Integer, Set<Integer>> ordering, List<List<Integer>> updates) {
    }
}
