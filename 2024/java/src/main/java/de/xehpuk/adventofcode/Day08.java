package de.xehpuk.adventofcode;

import de.xehpuk.adventofcode.Utils.Grid;
import de.xehpuk.adventofcode.Utils.II;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.stream.Stream;

public class Day08 {
    public static long part1(final Stream<String> lines) {
        final var grid = Utils.parseGrid(lines);
        return countAntinodes(grid);
    }

    private static long countAntinodes(final Grid grid) {
        final var antennas = Utils.mapByR(grid.elements(), ArrayList::new);
        final var antinodes = new HashSet<II>();
        for (var positions : antennas.values()) {
            for (int i = 0; i < positions.size(); i++) {
                final var position = positions.get(i);
                for (int j = i + 1; j < positions.size(); j++) {
                    final var position2 = positions.get(j);
                    final var diffX = Math.abs(position.l() - position2.l());
                    final var diffY = Math.abs(position.r() - position2.r());
                    if (position.l() < position2.l()) {
                        if (position.r() < position2.r()) {
                            antinodes.add(new II(position.l() - diffX, position.r() - diffY));
                            antinodes.add(new II(position2.l() + diffX, position2.r() + diffY));
                        } else {
                            antinodes.add(new II(position.l() - diffX, position.r() + diffY));
                            antinodes.add(new II(position2.l() + diffX, position2.r() - diffY));
                        }
                    } else {
                        if (position.r() < position2.r()) {
                            antinodes.add(new II(position.l() + diffX, position.r() - diffY));
                            antinodes.add(new II(position2.l() - diffX, position2.r() + diffY));
                        } else {
                            antinodes.add(new II(position.l() + diffX, position.r() + diffY));
                            antinodes.add(new II(position2.l() - diffX, position2.r() - diffY));
                        }
                    }
                }
            }
        }
        return antinodes.stream()
                .filter(antinode -> antinode.l() >= 0 && antinode.r() >= 0 &&
                                    antinode.l() < grid.width() && antinode.r() < grid.height())
                .count();
    }

    public static long part2(final Stream<String> lines) {
        final var grid = Utils.parseGrid(lines);
        return countAntinodes2(grid);
    }

    private static long countAntinodes2(final Grid grid) {
        final var antennas = Utils.mapByR(grid.elements(), ArrayList::new);
        final var antinodes = new HashSet<II>();
        for (var positions : antennas.values()) {
            for (int i = 0; i < positions.size(); i++) {
                final var position = positions.get(i);
                for (int j = i + 1; j < positions.size(); j++) {
                    final var position2 = positions.get(j);
                    final var diffX = Math.abs(position.l() - position2.l());
                    final var diffY = Math.abs(position.r() - position2.r());
                    if (position.l() < position2.l()) {
                        if (position.r() < position2.r()) {
                            for (int k = position2.r(), l = position2.l(); k < grid.height() && l < grid.width(); k += diffY, l += diffX) {
                                antinodes.add(new II(l, k));
                            }
                            for (int k = position.r(), l = position.l(); k >= 0 && l >= 0; k -= diffY, l -= diffX) {
                                antinodes.add(new II(l, k));
                            }
                        } else {
                            for (int k = position.r(), l = position.l(); k < grid.height() && l >= 0; k += diffY, l -= diffX) {
                                antinodes.add(new II(l, k));
                            }
                            for (int k = position2.r(), l = position2.l(); k >= 0 && l < grid.width(); k -= diffY, l += diffX) {
                                antinodes.add(new II(l, k));
                            }
                        }
                    } else {
                        if (position.r() < position2.r()) {
                            for (int k = position2.r(), l = position2.l(); k < grid.height() && l >= 0; k += diffY, l -= diffX) {
                                antinodes.add(new II(l, k));
                            }
                            for (int k = position.r(), l = position.l(); k >= 0 && l < grid.width(); k -= diffY, l += diffX) {
                                antinodes.add(new II(l, k));
                            }
                        } else {
                            for (int k = position.r(), l = position.l(); k < grid.height() && l < grid.width(); k += diffY, l += diffX) {
                                antinodes.add(new II(l, k));
                            }
                            for (int k = position2.r(), l = position2.l(); k >= 0 && l >= 0; k -= diffY, l -= diffX) {
                                antinodes.add(new II(l, k));
                            }
                        }
                    }
                }
            }
        }
        return antinodes.size();
    }
}
