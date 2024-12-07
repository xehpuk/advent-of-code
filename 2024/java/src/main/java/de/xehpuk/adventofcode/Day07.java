package de.xehpuk.adventofcode;

import java.util.Arrays;
import java.util.function.Predicate;
import java.util.stream.Stream;

public class Day07 {
    public static long part1(final Stream<String> lines) {
        return part(lines, Day07::test1);
    }

    public static long part2(final Stream<String> lines) {
        return part(lines, Day07::test2);
    }

    private static long part(final Stream<String> lines, final Predicate<Equation> test) {
        return lines
                .map(Day07::parseLine)
                .filter(test)
                .mapToLong(Equation::test)
                .sum();
    }

    private static boolean test1(final Equation equation) {
        return test(new Test(equation, new Op[]{Op.ADD, Op.MUL}));
    }

    private static boolean test2(final Equation equation) {
        return test(new Test(equation, new Op[]{Op.ADD, Op.MUL, Op.CONCAT}));
    }

    private static boolean test(final Test test) {
        return test(test, test.operands()[0], 1);
    }

    private static boolean test(final Test test, final long result, final int i) {
        if (test.operands().length == i) {
            return test.test() == result;
        }
        if (test.test() < result) {
            return false;
        }
        for (final var operator : test.operators()) {
            if (test(test, operator.eval(result, test.operands()[i]), i + 1)) {
                return true;
            }
        }
        return false;
    }

    record Test(long test, int[] operands, Op[] operators) {
        Test(final Equation equation, final Op[] operators) {
            this(equation.test(), equation.operands(), operators);
        }
    }

    private static Equation parseLine(final String line) {
        final String[] split = line.split(": ", 2);
        final var test = Long.parseLong(split[0]);
        final var operands = Arrays.stream(split[1].split(" "))
                .mapToInt(Integer::parseInt)
                .toArray();
        return new Equation(test, operands);
    }

    record Equation(long test, int[] operands) {
    }

    enum Op {
        ADD {
            @Override
            long eval(final long first, final int second) {
                return first + second;
            }
        },
        MUL {
            @Override
            long eval(final long first, final int second) {
                return first * second;
            }
        },
        CONCAT {
            @Override
            long eval(final long first, final int second) {
                final int digits = (int) (Math.log10(second) + 1);
                return first * (long) Math.pow(10, digits) + second;
            }
        };

        abstract long eval(long first, int second);
    }
}
