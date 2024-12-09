package de.xehpuk.adventofcode;

import java.util.*;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Stream;

public class Day08 {
    private static final Pattern P = Pattern.compile("[^.]");

    public static long part1(final Stream<String> lines) {
        final var freqMap = parseLines(lines);
        return countAntinodes(freqMap);
    }

    private static long countAntinodes(FreqMap freqMap) {
        Set<Position> antinodes = new HashSet<>();
        for (var entry : freqMap.antennas.entrySet()) {
            var positions = entry.getValue();
            for (int i = 0; i < positions.size(); i++) {
                var position = positions.get(i);
                for (int j = i + 1; j < positions.size(); j++) {
                    var position2 = positions.get(j);
                    var diffX = Math.abs(position.x - position2.x);
                    var diffY = Math.abs(position.y - position2.y);
                    if (position.x < position2.x) {
                        if (position.y < position2.y) {
                            antinodes.add(new Position(position.x - diffX, position.y - diffY));
                            antinodes.add(new Position(position2.x + diffX, position2.y + diffY));
                        } else {
                            antinodes.add(new Position(position.x - diffX, position.y + diffY));
                            antinodes.add(new Position(position2.x + diffX, position2.y - diffY));
                        }
                    } else {
                        if (position.y < position2.y) {
                            antinodes.add(new Position(position.x + diffX, position.y - diffY));
                            antinodes.add(new Position(position2.x - diffX, position2.y + diffY));
                        } else {
                            antinodes.add(new Position(position.x + diffX, position.y + diffY));
                            antinodes.add(new Position(position2.x - diffX, position2.y - diffY));
                        }
                    }
                }
            }
        }
        return antinodes.stream()
                .filter(antinode -> antinode.x >= 0 && antinode.y >= 0 &&
                                    antinode.x < freqMap.width && antinode.y < freqMap.height)
                .count();
    }

    private static long countAntinodes2(FreqMap freqMap) {
        Set<Position> antinodes = new HashSet<>();
        for (var entry : freqMap.antennas.entrySet()) {
            var positions = entry.getValue();
            for (int i = 0; i < positions.size(); i++) {
                var position = positions.get(i);
                for (int j = i + 1; j < positions.size(); j++) {
                    var position2 = positions.get(j);
                    var diffX = Math.abs(position.x - position2.x);
                    var diffY = Math.abs(position.y - position2.y);
                    if (position.x < position2.x) {
                        if (position.y < position2.y) {
                            for (int k = position2.y, l = position2.x; k < freqMap.height && l < freqMap.width; k += diffY, l += diffX) {
                                antinodes.add(new Position(l, k));
                            }
                            for (int k = position.y, l = position.x; k >= 0 && l >= 0; k -= diffY, l -= diffX) {
                                antinodes.add(new Position(l, k));
                            }
                        } else {
                            for (int k = position.y, l = position.x; k < freqMap.height && l >= 0; k += diffY, l -= diffX) {
                                antinodes.add(new Position(l, k));
                            }
                            for (int k = position2.y, l = position2.x; k >= 0 && l < freqMap.width; k -= diffY, l += diffX) {
                                antinodes.add(new Position(l, k));
                            }
                        }
                    } else {
                        if (position.y < position2.y) {
                            for (int k = position2.y, l = position2.x; k < freqMap.height && l >= 0; k += diffY, l -= diffX) {
                                antinodes.add(new Position(l, k));
                            }
                            for (int k = position.y, l = position.x; k >= 0 && l < freqMap.width; k -= diffY, l += diffX) {
                                antinodes.add(new Position(l, k));
                            }
                        } else {
                            for (int k = position.y, l = position.x; k < freqMap.height && l < freqMap.width; k += diffY, l += diffX) {
                                antinodes.add(new Position(l, k));
                            }
                            for (int k = position2.y, l = position2.x; k >= 0 && l >= 0; k -= diffY, l -= diffX) {
                                antinodes.add(new Position(l, k));
                            }
                        }
                    }
                }
            }
        }
        return antinodes.size();
    }

    public static long part2(final Stream<String> lines) {
        final var freqMap = parseLines(lines);
        return countAntinodes2(freqMap);
    }

    private static FreqMap parseLines(final Stream<String> lines) {
        final var antennas = new HashMap<Character, List<Position>>();

        Iterable<Utils.EI<String>> i = () -> lines
                .gather(Utils.indexed())
                .iterator();
        int width = 0;
        int height = 0;
        for (var ie : i) {
            width = Math.max(width, ie.e().length());
            height = ie.i() + 1;

            Iterable<MatchResult> p = () -> P.matcher(ie.e()).results().iterator();

            for (var m : p) {
                antennas.merge(m.group().charAt(0), new ArrayList<>(List.of(new Position(m.start(), ie.i()))),
                        (o, o2) -> {
                            o.addAll(o2);
                            return o;
                        });
            }
        }

        return new FreqMap(width, height, antennas);
//        return lines
//                .gather(Utils.indexed())
//                .flatMap(s -> P.matcher(s.e()).results()
//                        .map(result -> new Utils.EI<>(s.i(), result)))
//                .collect(Collectors.toMap(
//                        ie -> ie.e().group().charAt(0),
//                        ie -> new ArrayList<>(List.of(new Position(ie.e().start(), ie.i()))),
//                        (o, o2) -> {
//                            o.addAll(o2);
//                            return o;
//                        }));
    }

//    private static Map<Character, List<Position>> parseLines(final Stream<String> lines) {
//        return lines
//                .gather(Utils.indexed())
//                .flatMap(s -> P.matcher(s.e()).results()
//                        .map(result -> new Utils.EI<>(s.i(), result)))
//                .collect(Collectors.toMap(
//                        ie -> ie.e().group().charAt(0),
//                        ie -> new ArrayList<>(List.of(new Position(ie.e().start(), ie.i()))),
//                        (o, o2) -> {
//                            o.addAll(o2);
//                            return o;
//                        }));
//    }

    record FreqMap(int width, int height, Map<Character, List<Position>> antennas) {
    }

    record Position(int x, int y) {
    }
}
