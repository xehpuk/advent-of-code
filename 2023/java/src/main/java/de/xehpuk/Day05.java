package de.xehpuk;

import java.util.*;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Stream;

public class Day05 {
    private static final Pattern NUMBER_REGEX = Pattern.compile("\\d+");

    record Almanac(List<Long> seeds, List<NavigableMap<Long, DestinationAndLength>> conversionMaps) {
    }

    record DestinationAndLength(long destination, long length) {
    }

    public static long part1(Stream<String> lines) {
        Almanac almanac = parseAlmanac(lines, true);
        return almanac.seeds()
                .parallelStream()
                .mapToLong(seed -> findDestination(seed, almanac))
                .min()
                .getAsLong();
    }

    public static long part2(Stream<String> lines) {
        Almanac almanac = parseAlmanac(lines, false);

        NavigableMap<Long, Long> seeds = new TreeMap<>();
        {
            Iterator<Long> iterator = almanac.seeds().iterator();
            while (iterator.hasNext()) {
                long start = iterator.next();
                long length = iterator.next();
                seeds.put(start, start + length);
            }
        }

        return Utils.infiniteLongStream().filter(sourceId -> {
            long seed = findDestination(sourceId, almanac);
            var seedRange = seeds.floorEntry(seed);
            return seedRange != null && seed < seedRange.getValue();
        }).findFirst().getAsLong();
    }

    private static long findDestination(long sourceId, Almanac almanac) {
        return almanac.conversionMaps().stream().reduce(sourceId, (value, conversionMap) -> {
            var conversion = conversionMap.floorEntry(value);
            if (conversion == null) {
                return value;
            }
            long sourceStart = conversion.getKey();
            long sourceEnd = sourceStart + conversion.getValue().length();
            if (sourceEnd < value) {
                return value;
            }
            return value - sourceStart + conversion.getValue().destination();
        }, Utils.dummyCombiner());
    }

    private static Almanac parseAlmanac(Stream<String> lines, boolean forward) {
        return lines.reduce(new Almanac(List.of(), new ArrayList<>()), (value, line) -> {
            if (value.seeds.isEmpty()) {
                return new Almanac(List.copyOf(extractNumbers(line)), value.conversionMaps());
            }
            if (line.isEmpty()) {
                if (forward) {
                    value.conversionMaps().add(new TreeMap<>());
                } else {
                    value.conversionMaps().addFirst(new TreeMap<>());
                }
                return value;
            }
            if (line.endsWith(":")) {
                return value;
            }
            List<Long> numbers = extractNumbers(line);
            long destinationId = numbers.get(0);
            long sourceId = numbers.get(1);
            long rangeLength = numbers.get(2);
            if (forward) {
                value.conversionMaps().getLast().put(sourceId, new DestinationAndLength(destinationId, rangeLength));
            } else {
                value.conversionMaps().getFirst().put(destinationId, new DestinationAndLength(sourceId, rangeLength));
            }
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
