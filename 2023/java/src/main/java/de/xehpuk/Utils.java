package de.xehpuk;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.function.Function;
import java.util.stream.Stream;

public class Utils {
    public static <R> R withLines(String fileName, Function<Stream<String>, R> handleLines) {
        try (Stream<String> lines = Files.lines(Path.of(Main.class.getResource(STR."/_input/\{fileName}.txt").toURI()))) {
            return handleLines.apply(lines);
        } catch (IOException | URISyntaxException e) {
            throw new RuntimeException(e);
        }
    }

    public static <U> U combine(U u1, U u2) {
        throw new RuntimeException("Combiner");
    }
}
