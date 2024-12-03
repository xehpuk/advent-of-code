package de.xehpuk;

import java.util.regex.Pattern;
import java.util.stream.Stream;

public class Day03 {
    private static final Pattern PATTERN = Pattern.compile("mul\\((\\d+),(\\d+)\\)");
    private static final Pattern PATTERN2 = Pattern.compile("mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\)");

    public static long part1(final Stream<String> lines) {
        return lines
                .flatMap(Day03::parseLine)
                .mapToInt(mul -> mul.left() * mul.right())
                .sum();
    }

    public static long part2(final Stream<String> lines) {
        final Iterable<Instruction> instructions = () -> lines
                .flatMap(Day03::parseLine2)
                .iterator();

        var enabled = true;
        var sum = 0;
        for (var instruction : instructions) {
            switch (instruction) {
                case Mul(int left, int right):
                    if (enabled) {
                        sum += left * right;
                    }
                    break;
                case Do.INSTANCE:
                    enabled = true;
                    break;
                case Dont.INSTANCE:
                    enabled = false;
                    break;
            }
        }
        return sum;
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
