package de.xehpuk.adventofcode;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Function;
import java.util.function.Supplier;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Gatherer;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class Utils {
    public static <R> R withLines(final String fileName, final Function<Stream<String>, R> handleLines) {
        try (final var lines = Files.lines(Path.of(Utils.class.getResource("/input/%s.txt".formatted(fileName)).toURI()))) {
            return handleLines.apply(lines);
        } catch (IOException | URISyntaxException e) {
            throw new RuntimeException(e);
        }
    }

    public record LE<E>(E e, long l) {
    }

    public static <T> LE<T> withTiming(final Supplier<T> supplier) {
        final long start = System.currentTimeMillis();
        return new LE<>(supplier.get(), System.currentTimeMillis() - start);
    }

    public record II(int l, int r) {
    }

    public static <T> T middleElement(final List<T> list) {
        return list.get(list.size() / 2);
    }

    public static int countDigits(final int digits) {
        return switch (digits) {
            case int _ when digits < 10 -> 1;
            case int _ when digits < 100 -> 2;
            case int _ when digits < 1_000 -> 3;
            case int _ when digits < 10_000 -> 4;
            case int _ when digits < 100_000 -> 5;
            case int _ when digits < 1_000_000 -> 6;
            case int _ when digits < 10_000_000 -> 7;
            case int _ when digits < 100_000_000 -> 8;
            case int _ when digits < 1_000_000_000 -> 9;
            default -> 10;
        };
    }

    public static int countDigits(final long digits) {
        return switch (digits) {
            case int i -> countDigits(i);
            case long _ when digits < 10_000_000_000L -> 10;
            case long _ when digits < 100_000_000_000L -> 11;
            case long _ when digits < 1_000_000_000_000L -> 12;
            case long _ when digits < 10_000_000_000_000L -> 13;
            case long _ when digits < 100_000_000_000_000L -> 14;
            case long _ when digits < 1_000_000_000_000_000L -> 15;
            case long _ when digits < 10_000_000_000_000_000L -> 16;
            case long _ when digits < 100_000_000_000_000_000L -> 17;
            case long _ when digits < 1_000_000_000_000_000_000L -> 18;
            default -> 19;
        };
    }

    public static long pow(final int base, final int exponent) {
        long result = 1;
        for (int e = 0; e < exponent; e++) {
            result *= base;
        }
        return result;
    }

    public record EI<E>(E e, int i) {
    }

    // https://mail.openjdk.org/pipermail/core-libs-dev/2024-December/136867.html
    public static <TR> Gatherer<TR, ?, EI<TR>> indexed() {
        return Gatherer.ofSequential(
                () -> new int[1],
                Gatherer.Integrator.ofGreedy((state, element, downstream) ->
                        downstream.push(new EI<>(element, state[0]++)))
        );
    }

    public static long toBase(final int number, final int base) {
        if (base < 2 || base > 10) {
            throw new IndexOutOfBoundsException(base);
        }

        long result = 0;
        long multiplier = 1;
        int n = number;

        while (n > 0) {
            final int digit = n % base;
            result += digit * multiplier;
            multiplier *= 10;
            n /= base;
        }

        return result;
    }

    public static int nthDigit(final long number, final int n) {
        return (int) (number / pow(10, n) % 10);
    }

    public record Pair<L, R>(L l, R r) {
    }

    public static <L, R> Stream<Stream<Pair<L, R>>> cartesianProduct(final List<L> left, final List<R> right) {
        return switch (left.size()) {
            case 0 -> Stream.empty();
            case 1 -> Stream.of(right.stream().map(r -> new Pair<>(left.getFirst(), r)));
            case int s -> IntStream.range(0, (int) pow(s, right.size()))
                    .mapToLong(i -> toBase(i, s))
                    .mapToObj(i -> right.stream()
                            .gather(indexed())
                            .map(r -> new Pair<>(left.get(nthDigit(i, r.i())), r.e())));
        };
    }

    private static final Pattern NO_DOT = Pattern.compile("[^.]");

    public record Grid(int width, int height, List<Pair<II, Character>> elements) {
    }

    public static Grid parseGrid(final Stream<String> lines) {
        final var width = new AtomicInteger();
        final var height = new AtomicInteger();
        final var list = lines.peek(line -> width.set(line.length()))
                .gather(indexed())
                .flatMap(line -> {
                    height.set(line.i() + 1);
                    return NO_DOT.matcher(line.e()).results()
                            .map(r -> new Pair<>(new II(r.start(), line.i()), r.group().charAt(0)));
                })
                .toList();

        return new Grid(width.get(), height.get(), list);
    }

    public static <L, R> Map<L, R> mapByL(final Collection<Pair<L, R>> elements) {
        return elements.stream()
                .collect(Collectors.toMap(Pair::l, Pair::r));
    }

    public static <L, R, C extends Collection<L>> Map<R, C> mapByR(final Collection<Pair<L, R>> elements, final Supplier<C> constructor) {
        return elements.stream()
                .collect(Collectors.toMap(Pair::r, e -> {
                    final C c = constructor.get();
                    c.add(e.l());
                    return c;
                }, (c, c2) -> {
                    c.addAll(c2);
                    return c;
                }));
    }
}
