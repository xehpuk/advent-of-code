package de.xehpuk.adventofcode;

import java.util.HashSet;
import java.util.List;
import java.util.NoSuchElementException;
import java.util.stream.Stream;

public class Day06 {
    public static long part1(final Stream<String> lines) {
        final var map = lines
                .toList();
        var guard = findGuard(map);

        final var visitedPositions = new HashSet<>();
        visitedPositions.add(guard.position);
        while (true) {
            var old = guard;
            guard = guard.move();
            if (guard.position.y < 0 || guard.position.y >= map.size()) {
                return visitedPositions.size();
            }
            final var row = map.get(guard.position.y);
            if (guard.position.x < 0 || guard.position.x >= row.length()) {
                return visitedPositions.size();
            }
            if (row.charAt(guard.position.x) == '#') {
                guard = old.turnRight();
                continue;
            }
            visitedPositions.add(guard.position);
        }
    }

    public static long part2(final Stream<String> lines) {
        final var map = lines
                .toList();
        final var guard = findGuard(map);

        var sum = 0;
        for (int y = 0; y < map.size(); y++) {
            final var row = map.get(y);
            for (int x = 0; x < row.length(); x++) {
                if (row.charAt(x) != '#' && loops(map, guard, new Position(x, y))) {
                    sum++;
                }
            }
        }
        return sum;
    }

    private static Guard findGuard(final List<String> map) {
        for (int y = 0; y < map.size(); y++) {
            final var row = map.get(y);
            for (int x = 0; x < row.length(); x++) {
                switch (row.charAt(x)) {
                    case '^':
                        return new Guard(new Position(x, y), Direction.UP);
                    case '>':
                        return new Guard(new Position(x, y), Direction.RIGHT);
                    case 'v':
                        return new Guard(new Position(x, y), Direction.DOWN);
                    case '<':
                        return new Guard(new Position(x, y), Direction.LEFT);
                }
            }
        }
        throw new NoSuchElementException();
    }

    private static boolean loops(final List<String> map, final Guard initialGuard, final Position obstruction) {
        var guard = initialGuard;
        final var visitedPositions = new HashSet<>();
        visitedPositions.add(guard);
        while (true) {
            var old = guard;
            guard = guard.move();
            if (!visitedPositions.add(guard)) {
                return true;
            }
            if (guard.position.y < 0 || guard.position.y >= map.size()) {
                return false;
            }
            final var row = map.get(guard.position.y);
            if (guard.position.x < 0 || guard.position.x >= row.length()) {
                return false;
            }
            if (row.charAt(guard.position.x) == '#' || guard.position.equals(obstruction)) {
                guard = old.turnRight();
            }
        }
    }

    enum Direction {
        UP(0, -1),
        RIGHT(1, 0),
        DOWN(0, 1),
        LEFT(-1, 0);

        private final int dx;
        private final int dy;

        Direction(int dx, int dy) {
            this.dx = dx;
            this.dy = dy;
        }
    }

    record Position(int x, int y) {
    }

    record Guard(Position position, Direction direction) {
        Guard move() {
            return new Guard(new Position(position.x + direction.dx, position.y + direction.dy), direction);
        }

        Guard turnRight() {
            return new Guard(new Position(position.x, position.y), switch (direction) {
                case UP -> Direction.RIGHT;
                case RIGHT -> Direction.DOWN;
                case DOWN -> Direction.LEFT;
                case LEFT -> Direction.UP;
            });
        }
    }
}
