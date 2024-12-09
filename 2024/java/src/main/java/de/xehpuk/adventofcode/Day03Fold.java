package de.xehpuk.adventofcode;

import java.util.regex.Pattern;
import java.util.stream.Gatherers;
import java.util.stream.Stream;

/// An alternative to [Day03] using [java.util.stream.Gatherers]`.fold()` instead of a traditional `for`-loop.
public class Day03Fold {
    private static final Pattern PATTERN = Pattern.compile("mul\\((\\d+),(\\d+)\\)");
    private static final Pattern PATTERN2 = Pattern.compile("mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\)");

    public static long part1(final Stream<String> lines) {
        return lines
                .flatMap(Day03Fold::parseLine)
                .mapToInt(mul -> mul.m() * mul.n())
                .sum();
    }

    public static long part2(final Stream<String> lines) {
        record Acc(boolean enabled, long sum) {
        }

        return lines
                .flatMap(Day03Fold::parseLine2)
                .gather(Gatherers.fold(
                        () -> new Acc(true, 0),
                        (acc, instruction) ->
                                switch (instruction) {
                                    case Mul(int m, int n) -> acc.enabled()
                                            ? new Acc(true, acc.sum() + m * n)
                                            : acc;
                                    case Do.INSTANCE -> new Acc(true, acc.sum());
                                    case Dont.INSTANCE -> new Acc(false, acc.sum());
                                }
                ))
                .findAny().get().sum();
    }

    private static Stream<Mul> parseLine(final String line) {
        return PATTERN.matcher(line).results()
                .map(result -> new Mul(Integer.parseInt(result.group(1)), Integer.parseInt(result.group(2))));
    }

    private static Stream<Instruction> parseLine2(final String line) {
        return PATTERN2.matcher(line).results()
                .map(result -> switch (result.group()) {
                    default -> new Mul(Integer.parseInt(result.group(1)), Integer.parseInt(result.group(2)));
                    case "do()" -> Do.INSTANCE;
                    case "don't()" -> Dont.INSTANCE;
                });
    }

    sealed interface Instruction permits Mul, Do, Dont {
    }

    record Mul(int m, int n) implements Instruction {
    }

    enum Do implements Instruction {
        INSTANCE
    }

    enum Dont implements Instruction {
        INSTANCE
    }
}
