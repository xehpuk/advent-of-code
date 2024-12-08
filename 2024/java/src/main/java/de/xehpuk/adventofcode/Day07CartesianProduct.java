package de.xehpuk.adventofcode;

import java.util.Arrays;
import java.util.List;
import java.util.function.Predicate;
import java.util.stream.Gatherer;
import java.util.stream.Gatherers;
import java.util.stream.Stream;

public class Day07CartesianProduct {
    public static long part1(final Stream<String> lines) {
        return part(lines, Day07CartesianProduct::test1);
    }

    public static long part2(final Stream<String> lines) {
        return part(lines, Day07CartesianProduct::test2);
    }

    private static long part(final Stream<String> lines, final Predicate<Equation> test) {
        return lines
                .map(Day07CartesianProduct::parseLine)
                .filter(test)
                .mapToLong(Equation::test)
                .sum();
    }

    private static boolean test1(final Equation equation) {
        return test(new Test(equation, List.of(Op.ADD, Op.MUL)));
    }

    private static boolean test2(final Equation equation) {
        return test(new Test(equation, List.of(Op.ADD, Op.MUL, Op.CONCAT)));
    }

    private static boolean test(final Test test) {
        return Utils.cartesianProduct(test.operators(), test.operands().subList(1, test.operands().size()))
                .parallel()
                .gather(Gatherer.<Stream<Utils.Pair<Op, Integer>>, Boolean>of(
                        Gatherer.Integrator.ofGreedy((_, pairs, downstream) -> {
                            final long result = pairs
                                    .gather(Gatherers.fold(
                                            () -> (long) test.operands().getFirst(),
                                            (acc, pair) -> pair.l().eval(acc, pair.r())
                                    ))
                                    .findAny().get();
                            if (result == test.test()) {
                                downstream.push(true);
                                return false;
                            }
                            return true;
                        }),
                        (_, downstream) -> downstream.push(false)
                ))
                .findFirst().get();
    }

    record Test(long test, List<Integer> operands, List<Op> operators) {
        Test(final Equation equation, final List<Op> operators) {
            this(equation.test(), equation.operands(), operators);
        }
    }

    private static Equation parseLine(final String line) {
        final String[] split = line.split(": ", 2);
        final var test = Long.parseLong(split[0]);
        final var operands = Arrays.stream(split[1].split(" "))
                .map(Integer::parseInt)
                .toList();
        return new Equation(test, operands);
    }

    record Equation(long test, List<Integer> operands) {
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
                return first * Utils.pow(10, Utils.countDigits(second)) + second;
            }
        };

        abstract long eval(long first, int second);
    }
}
