package de.xehpuk.adventofcode;

import de.xehpuk.adventofcode.Utils.Direction;
import de.xehpuk.adventofcode.Utils.II;

import java.util.HashSet;
import java.util.List;
import java.util.NoSuchElementException;
import java.util.Set;
import java.util.stream.Stream;

public class Day06 {
    public static long part1(final Stream<String> lines) {
        final var map = lines.toList();
        final var guard = findGuard(map);

        return visitedPositionsBeforeLeave(map, guard).size();
    }

    public static long part2(final Stream<String> lines) {
        final var map = lines.toList();
        final var guard = findGuard(map);

        return visitedPositionsBeforeLeave(map, guard).stream()
                .filter(obstruction -> loops(map, guard, obstruction))
                .count();
    }

    private static Set<II> visitedPositionsBeforeLeave(final List<String> map, final Guard initialGuard) {
        var guard = initialGuard;
        final var visitedPositions = new HashSet<II>();
        visitedPositions.add(guard.position);
        while (true) {
            var old = guard;
            guard = guard.move();
            if (guard.position.r() < 0 || guard.position.r() >= map.size()) {
                return visitedPositions;
            }
            final var row = map.get(guard.position.r());
            if (guard.position.l() < 0 || guard.position.l() >= row.length()) {
                return visitedPositions;
            }
            if (row.charAt(guard.position.l()) == '#') {
                guard = old.turnRight();
                continue;
            }
            visitedPositions.add(guard.position);
        }
    }

    private static Guard findGuard(final List<String> map) {
        for (int y = 0; y < map.size(); y++) {
            final var row = map.get(y);
            for (int x = 0; x < row.length(); x++) {
                switch (row.charAt(x)) {
                    case '^':
                        return new Guard(new II(x, y), Direction.UP);
                    case '>':
                        return new Guard(new II(x, y), Direction.RIGHT);
                    case 'v':
                        return new Guard(new II(x, y), Direction.DOWN);
                    case '<':
                        return new Guard(new II(x, y), Direction.LEFT);
                }
            }
        }
        throw new NoSuchElementException();
    }

    private static boolean loops(final List<String> map, final Guard initialGuard, final II obstruction) {
        var guard = initialGuard;
        final var visitedPositions = new HashSet<>();
        visitedPositions.add(guard);
        while (true) {
            var old = guard;
            guard = guard.move();
            if (!visitedPositions.add(guard)) {
                return true;
            }
            if (guard.position.r() < 0 || guard.position.r() >= map.size()) {
                return false;
            }
            final var row = map.get(guard.position.r());
            if (guard.position.l() < 0 || guard.position.l() >= row.length()) {
                return false;
            }
            if (row.charAt(guard.position.l()) == '#' || guard.position.equals(obstruction)) {
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
