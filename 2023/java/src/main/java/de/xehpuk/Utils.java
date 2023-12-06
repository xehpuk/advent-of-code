package de.xehpuk;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.function.BinaryOperator;
import java.util.function.Function;
import java.util.stream.LongStream;
import java.util.stream.Stream;

public class Utils {
    public static <R> R withLines(String fileName, Function<Stream<String>, R> handleLines) {
        try (Stream<String> lines = Files.lines(Path.of(Main.class.getResource(STR."/_input/\{fileName}.txt").toURI()))) {
            return handleLines.apply(lines);
        } catch (IOException | URISyntaxException e) {
            throw new RuntimeException(e);
        }
    }

    @SuppressWarnings("unchecked")
    public static <U> BinaryOperator<U> dummyCombiner() {
        return (BinaryOperator<U>) DUMMY_COMBINER;
    }

    public static LongStream infiniteLongStream() {
        return LongStream.iterate(0, i -> i + 1).parallel();
    }

    private static final BinaryOperator<?> DUMMY_COMBINER = (_, _) -> {
        throw new Error("combiner");
    };
}
