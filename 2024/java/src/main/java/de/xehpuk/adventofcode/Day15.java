package de.xehpuk.adventofcode;

import de.xehpuk.adventofcode.Utils.EI;
import de.xehpuk.adventofcode.Utils.Grid;
import de.xehpuk.adventofcode.Utils.II;
import de.xehpuk.adventofcode.Utils.Pair;

import java.util.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Gatherer;
import java.util.stream.Gatherer.Downstream;
import java.util.stream.Gatherer.Integrator;
import java.util.stream.Stream;

import static de.xehpuk.adventofcode.Utils.*;

public class Day15 {
    public static long part1(final Stream<String> lines) {
        Pair<Grid, String> pair = parseLines(lines);
        Grid grid = pair.l();
        System.out.println(grid);
        String moves = pair.r();
        Map<Character, HashSet<II>> map = groupByR(grid.elements(), HashSet::new);
        II robotPos = map.get('@').iterator().next();
        Set<II> walls = map.get('#');
        Set<II> boxes = map.get('O');
        x: for (final Utils.Direction direction : moves.chars().mapToObj(c -> Utils.Direction.fromChar((char) c)).toList()) {
            II newRobotPos = direction.move(robotPos);
            if (walls.contains(newRobotPos)) {
                continue;
            }
            if (boxes.contains(newRobotPos)) {
                II check = newRobotPos;
                while (true) {
                    if (walls.contains(check)) {
                        continue x;
                    }
                    if (!boxes.contains(check)) {
                        break;
                    }
                    check = direction.move(check);
                }
                boxes.remove(newRobotPos);
                boxes.add(check);
            }
            robotPos = newRobotPos;
        }
        return boxes.stream()
                .mapToLong(box -> box.l() + 100L * box.r())
                .sum();
    }

    public static long part2(final Stream<String> lines) {
        return 0L;
    }

    public static Pair<Grid, String> parseLines(final Stream<String> lines) {
        final var width = new AtomicInteger();
        final var height = new AtomicInteger();

        return (Pair<Grid, String>) lines
                .gather(indexed())
                .gather(Gatherer.ofSequential(
                        () -> new Object[]{new ArrayList<>(), new StringBuilder(), false},
                        Integrator.ofGreedy((Object[] state, EI<String> line, Downstream<? super Object> _) -> {
                            if (line.e().isEmpty()) {
                                state[2] = true;
                                return true;
                            }
                            if ((boolean) state[2]) {
                                ((StringBuilder) state[1]).append(line.e());
                                return true;
                            }
                            width.set(line.e().length());
                            height.set(line.i() + 1);
                            NO_DOT.matcher(line.e()).results()
                                    .map(r -> new Pair<>(new II(r.start(), line.i()), r.group().charAt(0)))
                                    .forEach(((List<Pair<II, Character>>) state[0])::add);
                            return true;

                        }),
                        (state, downstream) -> downstream.push(new Pair<>(new Grid(width.get(), height.get(), (List<Pair<II, Character>>) state[0]), state[1].toString())))
                )
                .findAny().get();
    }
}
