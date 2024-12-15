package de.xehpuk.adventofcode;

import de.xehpuk.adventofcode.Utils.II;

import java.util.Arrays;
import java.util.List;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Gatherers;
import java.util.stream.Stream;

public class Day14 {
    private static final int WIDTH = 101;
    private static final int HEIGHT = 103;

    private static final Pattern INTS = Pattern.compile("-?\\d+");

    public static long part1(final Stream<String> lines) {
        return part1(lines, WIDTH, HEIGHT);
    }

    static long part1(final Stream<String> lines, final int width, final int height) {
        final int middleX = width / 2;
        final int middleY = height / 2;
        return lines.map(Day14::parseLine)
                .map(robot -> robot.p().add(robot.v().mul(100)).mod(width, height))
                .gather(Gatherers.fold(
                        () -> new int[4],
                        (quadrants, p) -> {
                            if (p.l() != middleX && p.r() != middleY) {
                                if (p.l() < middleX) {
                                    if (p.r() < middleY) {
                                        quadrants[0]++;
                                    } else {
                                        quadrants[1]++;
                                    }
                                } else {
                                    if (p.r() < middleY) {
                                        quadrants[2]++;
                                    } else {
                                        quadrants[3]++;
                                    }
                                }
                            }
                            return quadrants;
                        }
                ))
                .flatMapToInt(Arrays::stream)
                .asLongStream()
                .reduce(1L, (a, b) -> a * b);
    }

    public static long part2(final Stream<String> lines) {
        var robots = lines.map(Day14::parseLine).toList();
        for (int i = 0; ; i++) {
            if (looksLikeXmasTree(robots)) {
                return i;
            }
            robots = robots.stream()
                    .map(robot -> new Robot(robot.p().add(robot.v()).mod(WIDTH, HEIGHT), robot.v()))
                    .toList();
        }
    }

    /**
     * Randomly checks whether the robots don't overlap. How did part 2 pass QA?
     */
    private static boolean looksLikeXmasTree(final List<Robot> robots) {
        final int width = WIDTH;
        final int height = HEIGHT;
        final var ps = robots.stream()
                .map(Robot::p)
                .collect(Collectors.toSet());
        final boolean xmasTree = robots.size() == ps.size();
        if (xmasTree) {
            final var builder = new StringBuilder((width + 1) * height);
            for (int y = 0; y < height; y++) {
                for (int x = 0; x < width; x++) {
                    builder.append(ps.contains(new II(x, y)) ? 'R' : '.');
                }
                builder.append('\n');
            }
            System.out.println(builder);
        }
        return xmasTree;
    }

    private static Robot parseLine(final String line) {
        final int[] ints = INTS.matcher(line)
                .results()
                .limit(4)
                .map(MatchResult::group)
                .mapToInt(Integer::parseInt)
                .toArray();
        return new Robot(new II(ints[0], ints[1]), new II(ints[2], ints[3]));
    }

    private record Robot(II p, II v) {
    }
}
