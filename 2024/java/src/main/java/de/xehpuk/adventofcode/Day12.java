package de.xehpuk.adventofcode;

import de.xehpuk.adventofcode.Utils.Direction;
import de.xehpuk.adventofcode.Utils.II;

import java.util.HashSet;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Set;
import java.util.stream.Stream;

public class Day12 {
    public static long part1(final Stream<String> lines) {
        final var grid = Utils.parseGrid(lines);
        final var map = Utils.groupByL(grid.elements());
        final var iterator = map.entrySet().iterator();
        final var visitedPlots = new HashSet<II>();

        long price = 0;
        while (visitedPlots.size() < map.size()) {
            price += processRegion(iterator.next(), map, visitedPlots, new II(0, 0)).product();
        }
        return price;
    }

    private static II processRegion(final Entry<II, Character> entry, final Map<II, Character> map, final Set<II> visitedPlots, final II areaPerimeter) {
        final var plot = entry.getKey();
        if (!visitedPlots.add(plot)) {
            return areaPerimeter;
        }
        final var plant = entry.getValue();
        var result = new II(1, 0);
        for (final var direction : Direction.values()) {
            final var neighborPlot = direction.move(plot);
            final var neighborPlant = map.get(neighborPlot);
            if (neighborPlant != plant) { // ASCII in `CharacterCache`
                result = result.addR(1);
                continue;
            }
            result = result.add(processRegion(Map.entry(neighborPlot, neighborPlant), map, visitedPlots, areaPerimeter));
        }
        return areaPerimeter.add(result);
    }

    public static long part2(final Stream<String> lines) {
        return 0L;
    }
}
