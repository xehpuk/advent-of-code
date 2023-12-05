package de.xehpuk;

import java.util.*;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.LongStream;
import java.util.stream.Stream;

public class Day05 {
    private static final Pattern NUMBER_REGEX = Pattern.compile("\\d+");

    record Almanac(List<Long> seeds, List<NavigableMap<Long, SourceAndLength>> conversionMaps) {
    }

    record SourceAndLength(long source, long length) {
    }

    public static long part1(Stream<String> lines) {
        Almanac almanac = parseAlmanac(lines);

        return LongStream.iterate(0, i -> i + 1).filter(destinationId -> {
            long seed = almanac.conversionMaps().reversed().stream().reduce(destinationId, (value, conversionMap) -> {
                var conversion = conversionMap.floorEntry(value);
                if (conversion == null) {
                    return value;
                }
                long destinationStart = conversion.getKey();
                long destinationEnd = destinationStart + conversion.getValue().length();
                if (destinationEnd < value) {
                    return value;
                }
                return value - destinationStart + conversion.getValue().source();
            }, Utils::combine);
            return almanac.seeds().contains(seed);
        }).findFirst().getAsLong();
    }

    private static Almanac parseAlmanac(Stream<String> lines) {
        return lines.reduce(new Almanac(List.of(), new ArrayList<>()), (value, line) -> {
            if (value.seeds.isEmpty()) {
                return new Almanac(List.copyOf(extractNumbers(line)), value.conversionMaps());
            }
            if (line.isEmpty()) {
                value.conversionMaps().add(new TreeMap<>());
                return value;
            }
            if (line.endsWith(":")) {
                return value;
            }
            List<Long> ids = extractNumbers(line);
            long destinationId = ids.get(0);
            long sourceId = ids.get(1);
            long rangeLength = ids.get(2);
            value.conversionMaps().getLast().put(destinationId, new SourceAndLength(sourceId, rangeLength));
            return value;
        }, Utils::combine);
    }

    private static List<Long> extractNumbers(String line) {
        return NUMBER_REGEX.matcher(line).results()
                .map(MatchResult::group)
                .map(Long::valueOf)
                .toList();
    }
}
