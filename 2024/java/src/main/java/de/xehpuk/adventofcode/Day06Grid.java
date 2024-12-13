package de.xehpuk.adventofcode;

import de.xehpuk.adventofcode.Utils.Direction;
import de.xehpuk.adventofcode.Utils.Grid;
import de.xehpuk.adventofcode.Utils.II;

import java.util.*;
import java.util.stream.Stream;

public class Day06Grid {
    public static long part1(final Stream<String> lines) {
        final var grid = Utils.parseGrid(lines);

        return visitedPositionsBeforeLeave(grid).size();
    }

    public static long part2(final Stream<String> lines) {
        final var grid = Utils.parseGrid(lines);

        return visitedPositionsBeforeLeave(grid).stream()
                .filter(obstruction -> loops(grid, obstruction))
                .count();
    }

    private static Set<II> visitedPositionsBeforeLeave(final Grid grid) {
        final var gridByChars = Utils.groupByR(grid.elements(), LinkedHashSet::new);
        var guard = findGuard(gridByChars);
        final var obstructions = gridByChars.get('#');
        final var visitedPositions = new HashSet<II>();
        visitedPositions.add(guard.position);
        while (true) {
            var old = guard;
            guard = guard.move();
            if (guard.position().r() < 0 || guard.position().r() >= grid.height()) {
                return visitedPositions;
            }
            if (guard.position().l() < 0 || guard.position().l() >= grid.width()) {
                return visitedPositions;
            }
            if (obstructions.contains(guard.position())) {
                guard = old.turnRight();
                continue;
            }
            visitedPositions.add(guard.position());
        }
    }

    private static <C extends SequencedCollection<II>> Guard findGuard(final Map<Character, C> gridByChars) {
        for (char c : List.of('^', '>', 'v', '<')) {
            final var pos = gridByChars.get(c);
            if (pos != null) {
                return new Guard(pos.getFirst(), Direction.fromChar(c));
            }
        }
        throw new NoSuchElementException();
    }

    private static boolean loops(final Grid grid, final II obstruction) {
        final var gridByChars = Utils.groupByR(grid.elements(), LinkedHashSet::new);
        final var obstructions = gridByChars.get('#');
        var guard = findGuard(gridByChars);
        final var visitedPositions = new HashSet<>();
        visitedPositions.add(guard);
        while (true) {
            var old = guard;
            guard = guard.move();
            if (!visitedPositions.add(guard)) {
                return true;
            }
            if (guard.position().r() < 0 || guard.position().r() >= grid.height()) {
                return false;
            }
            if (guard.position().l() < 0 || guard.position().l() >= grid.width()) {
                return false;
            }
            if (obstructions.contains(guard.position()) || guard.position().equals(obstruction)) {
                guard = old.turnRight();
            }
        }
    }

    record Guard(II position, Direction direction) {
        Guard move() {
            return new Guard(direction.move(position), direction);
        }

        Guard turnRight() {
            return new Guard(position, direction.rotateClockwise());
        }
    }
}
