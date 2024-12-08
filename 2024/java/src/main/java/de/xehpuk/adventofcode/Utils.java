package de.xehpuk.adventofcode;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.function.Function;
import java.util.stream.Gatherer;
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

    public static <T> T middleElement(final List<T> list) {
        return list.get(list.size() / 2);
    }
}
