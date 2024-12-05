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

    // https://mail.openjdk.org/pipermail/core-libs-dev/2024-December/136867.html
    public static <TR> Gatherer<TR, ?, IndexedElement<TR>> indexed() {
        return Gatherer.ofSequential(
                () -> new int[1],
                Gatherer.Integrator.ofGreedy((state, element, downstream) ->
                        downstream.push(new IndexedElement<>(state[0]++, element)))
        );
    }

    public record IndexedElement<T>(int index, T element) {
    }

    public static <T> T middleElement(final List<T> list) {
        return list.get(list.size() / 2);
    }
}
