package de.xehpuk;

import java.util.regex.Pattern;
import java.util.stream.Stream;

/// An alternative to [Day03] using `reduce()` instead of a traditional `for`-loop.
public class Day03Reduce {
    private static final Pattern PATTERN = Pattern.compile("mul\\((\\d+),(\\d+)\\)");
    private static final Pattern PATTERN2 = Pattern.compile("mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\)");

    public static long part1(final Stream<String> lines) {
        return lines
                .flatMap(Day03Reduce::parseLine)
                .mapToInt(mul -> mul.left() * mul.right())
                .sum();
    }

    public static long part2(final Stream<String> lines) {
        record Acc(boolean enabled, long sum) {
        }

        return lines
                .flatMap(Day03Reduce::parseLine2)
                .reduce(new Acc(true, 0),
                        (acc, instruction) ->
                                switch (instruction) {
                                    case Mul(int left, int right) -> acc.enabled()
                                            ? new Acc(true, acc.sum() + left * right)
                                            : acc;
                                    case Do.INSTANCE -> new Acc(true, acc.sum());
                                    case Dont.INSTANCE -> new Acc(false, acc.sum());
                                },
                        (acc, acc2) -> new Acc(acc2.enabled(), acc.sum() + acc2.sum())
                ).sum();
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

    record Mul(int left, int right) implements Instruction {
    }

    enum Do implements Instruction {
        INSTANCE
    }

    enum Dont implements Instruction {
        INSTANCE
    }
}
