package de.xehpuk.adventofcode;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.Collection;
import java.util.List;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Function;
import java.util.function.Supplier;
import java.util.function.ToLongFunction;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Gatherer;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class Utils {
    private static Stream<String> lines(final String fileName) throws IOException, URISyntaxException {
        return Files.lines(Path.of(Utils.class.getResource("/input/%s.txt".formatted(fileName)).toURI()));
    }

    public static <R> R withLines(final String fileName, final Function<Stream<String>, R> handleLines) {
        try (final var lines = lines(fileName)) {
            return handleLines.apply(lines);
        } catch (IOException | URISyntaxException e) {
            throw new RuntimeException(e);
        }
    }

    public static long withLines(final String fileName, final ToLongFunction<Stream<String>> handleLines) {
        try (final var lines = lines(fileName)) {
            return handleLines.applyAsLong(lines);
        } catch (IOException | URISyntaxException e) {
            throw new RuntimeException(e);
        }
    }

    public record LE<E>(E e, long l) {
        @Override
        public String toString() {
            return "(" + e + ", " + l + ")";
        }
    }

    public static <T> LE<T> withTiming(final Supplier<T> supplier) {
        final long start = System.currentTimeMillis();
        return new LE<>(supplier.get(), System.currentTimeMillis() - start);
    }

    public record LL(long l, long r) {
    }

    public record II(int l, int r) {
        public II add(final II other) {
            return add(other.l, other.r);
        }

        public II add(final int l, final int r) {
            return new II(this.l + l, this.r + r);
        }

        public II addL(final int l) {
            return new II(this.l + l, r);
        }

        public II addR(final int r) {
            return new II(l, this.r + r);
        }

        public int sum() {
            return l + r;
        }

        public int product() {
            return l * r;
        }

        public II mul(final int n) {
            return new II(l * n, r * n);
        }

        public II mod(final int l, final int r) {
            return new II(Math.floorMod(this.l, l), Math.floorMod(this.r, r));
        }

        @Override
        public String toString() {
            return "(" + l + ", " + r + ")";
        }
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
        @Override
        public String toString() {
            return "(" + e + ", " + i + ")";
        }
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

    public static int gcd(final int a, final int b) {
        return b == 0 ? a : gcd(b, a % b);
    }

    public static int lcm(final int a, final int b) {
        return b == 0 ? 0 : a * b / gcd(a, b);
    }

    public record Pair<L, R>(L l, R r) {
        @Override
        public String toString() {
            return "(" + l + ", " + r + ")";
        }
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
        @Override
        public String toString() {
            final var map = groupByL(elements);
            final var builder = new StringBuilder((width + 1) * height);
            for (int y = 0; y < height; y++) {
                for (int x = 0; x < width; x++) {
                    builder.append(map.getOrDefault(new II(x, y), '.'));
                }
                builder.append('\n');
            }
            return builder.toString();
        }
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

    public static <L, R> Map<L, R> groupByL(final Collection<Pair<L, R>> elements) {
        return elements.stream()
                .collect(Collectors.toMap(Pair::l, Pair::r));
    }

    public static <L, R, C extends Collection<L>> Map<R, C> groupByR(final Collection<Pair<L, R>> elements, final Supplier<C> constructor) {
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

    enum Direction {
        UP('^', 0, -1),
        RIGHT('>', 1, 0),
        DOWN('v', 0, 1),
        LEFT('<', -1, 0);

        static Direction fromChar(final char c) {
            return Arrays.stream(values())
                    .filter(direction -> direction.c == c)
                    .findAny().get();
        }

        private final char c;
        private final int dx;
        private final int dy;

        Direction(final char c, final int dx, final int dy) {
            this.c = c;
            this.dx = dx;
            this.dy = dy;
        }

        Direction rotateClockwise() {
            return switch (this) {
                case UP -> RIGHT;
                case RIGHT -> DOWN;
                case DOWN -> LEFT;
                case LEFT -> UP;
            };
        }

        II move(final II ii) {
            return ii.add(dx, dy);
        }
    }
}
