package de.xehpuk.adventofcode;

import de.xehpuk.adventofcode.Utils.II;

import java.util.ArrayList;
import java.util.function.Predicate;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Gatherer;
import java.util.stream.LongStream;
import java.util.stream.Stream;

public class Day13 {
    private static final Pattern INTS = Pattern.compile("\\d+");

    public static long part1(final Stream<String> lines) {
        return tokens(lines).sum();
    }

    public static long part2(final Stream<String> lines) {
        return tokens2(lines).sum();
    }

    static LongStream tokens(final Stream<String> lines) {
        return lines.filter(Predicate.not(String::isEmpty))
                .gather(parseMachine())
                .mapToLong(Day13::tokens);
    }

    static LongStream tokens2(final Stream<String> lines) {
        return lines.filter(Predicate.not(String::isEmpty))
                .gather(parseMachine())
                .mapToLong(Day13::tokens2);
    }

    private static Gatherer<String, ?, Machine> parseMachine() {
        return Gatherer.ofSequential(
                ArrayList<II>::new,
                Gatherer.Integrator.ofGreedy((state, element, downstream) -> {
                    final var array = INTS.matcher(element).results()
                            .limit(2)
                            .map(MatchResult::group)
                            .mapToInt(Integer::parseInt)
                            .toArray();
                    final var xy = new II(array[0], array[1]);
                    if (state.size() < 2) {
                        state.add(xy);
                        return true;
                    }
                    final var machine = new Machine(state.getFirst(), state.getLast(), xy);
                    state.clear();
                    return downstream.push(machine);
                })
        );
    }

    private static long tokens(final Machine machine) {
        var a = new II(machine.buttonA().l() * machine.buttonB().r(), machine.buttonA().r() * machine.buttonB().l());
        var b = new II(machine.buttonB().l() * machine.buttonB().r(), machine.buttonB().r() * machine.buttonB().l());
        var p = new II(machine.prize().l() * machine.buttonB().r(), machine.prize().r() * machine.buttonB().l());
        var r = new II(a.l() - a.r(), p.l() - p.r());
        var z = r.l() == 0 ? 1 : r.r() / r.l(); // a
        boolean solvable = z * r.l() == r.r();
        if (!solvable) {
            return 0;
        }
        int y = (machine.prize().l() - z * machine.buttonA().l()) / machine.buttonB().l();
        return 3 * z + y;
    }

    private static long tokens2(final Machine machine) {
        return 0;
    }

    private record Machine(II buttonA, II buttonB, II prize) {
    }
}
