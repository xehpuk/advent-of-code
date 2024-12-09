package de.xehpuk.adventofcode;

import java.util.regex.Pattern;
import java.util.stream.Gatherer;
import java.util.stream.Stream;

/// An alternative to [Day03] using [Gatherer]`.ofSequential()` instead of a traditional `for`-loop.
public class Day03Gatherer {
    private static final Pattern PATTERN = Pattern.compile("mul\\((\\d+),(\\d+)\\)");
    private static final Pattern PATTERN2 = Pattern.compile("mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\)");

    public static long part1(final Stream<String> lines) {
        return lines
                .flatMap(Day03Gatherer::parseLine)
                .mapToInt(mul -> mul.m() * mul.n())
                .sum();
    }

    public static long part2(final Stream<String> lines) {
        class Acc {
            boolean enabled = true;
            long sum;
        }

        return lines
                .flatMap(Day03Gatherer::parseLine2)
                .gather(Gatherer.<Instruction, Acc, Long>ofSequential(Acc::new, (acc, element, _) -> {
                    switch (element) {
                        case Mul(int m, int n):
                            if (acc.enabled) {
                                acc.sum += m * n;
                            }
                            break;
                        case Do.INSTANCE:
                            acc.enabled = true;
                            break;
                        case Dont.INSTANCE:
                            acc.enabled = false;
                            break;
                    }
                    return true;
                }, (acc, downstream) -> downstream.push(acc.sum)))
                .findAny().get();
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
