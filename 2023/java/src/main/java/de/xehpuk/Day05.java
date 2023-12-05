package de.xehpuk;

import java.util.*;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Stream;

public class Day05 {
    private static final Pattern NUMBER_REGEX = Pattern.compile("\\d+");

    record Almanac(List<Long> seeds, List<NavigableMap<Long, SourceAndLength>> conversionMaps) {
    }

    record SourceAndLength(long source, long length) {
    }

    public static long part1(Stream<String> lines) {
        Almanac almanac = parseAlmanac(lines);
        var seeds = Set.copyOf(almanac.seeds());

        return Utils.infiniteLongStream().filter(destinationId -> {
            long seed = findSource(destinationId, almanac);
            return seeds.contains(seed);
        }).findFirst().getAsLong();
    }

    public static long part2(Stream<String> lines) {
        Almanac almanac = parseAlmanac(lines);

        NavigableMap<Long, Long> seeds = new TreeMap<>();
        {
            Iterator<Long> iterator = almanac.seeds().iterator();
            while (iterator.hasNext()) {
                long start = iterator.next();
                long length = iterator.next();
                seeds.put(start, start + length);
            }
        }

        return Utils.infiniteLongStream().filter(destinationId -> {
            long seed = findSource(destinationId, almanac);
            var seedRange = seeds.floorEntry(seed);
            return seedRange != null && seed < seedRange.getValue();
        }).findFirst().getAsLong();
    }

    private static Long findSource(long destinationId, Almanac almanac) {
        return almanac.conversionMaps().reversed().stream().reduce(destinationId, (value, conversionMap) -> {
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
        }, Utils.dummyCombiner());
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
            List<Long> numbers = extractNumbers(line);
            long destinationId = numbers.get(0);
            long sourceId = numbers.get(1);
            long rangeLength = numbers.get(2);
            value.conversionMaps().getLast().put(destinationId, new SourceAndLength(sourceId, rangeLength));
            return value;
        }, Utils.dummyCombiner());
    }

    private static List<Long> extractNumbers(String line) {
        return NUMBER_REGEX.matcher(line).results()
                .map(MatchResult::group)
                .map(Long::valueOf)
                .toList();
    }
}
