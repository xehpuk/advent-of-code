import de.xehpuk.adventofcode.*;

void main() {
    solve(1, 1, Day01::part1);
    solve(1, 2, Day01::part2);
    solve(2, 1, Day02::part1);
    solve(2, 2, Day02::part2);
    solve(3, 1, Day03::part1);
    solve(3, 2, Day03::part2);
    solve(4, 1, Day04::part1);
    solve(4, 2, Day04::part2);
    solve(5, 1, Day05::part1);
    solve(5, 2, Day05::part2);
    solve(6, 1, Day06::part1);
    solve(6, 2, Day06::part2);
    solve(7, 1, Day07::part1);
    solve(7, 2, Day07::part2);
    solve(8, 1, Day08::part1);
    solve(8, 2, Day08::part2);
    solve(9, 1, Day09::part1);
    solve(9, 2, Day09::part2);
    solve(10, 1, Day10::part1);
    solve(10, 2, Day10::part2);
    solve(11, 1, Day11::part1);
    solve(11, 2, Day11::part2);
    solve(12, 1, Day12::part1);
    solve(12, 2, Day12::part2);
    solve(13, 1, Day13::part1);
    solve(13, 2, Day13::part2); // too high 106084147308293
    solve(14, 1, Day14::part1);
    solve(14, 2, Day14::part2);
}

void solve(final int day, final int part, final Function<Stream<String>, ?> handleLines) {
    final var fmtDay = "%02d".formatted(day);
    final var solution = Utils.withTiming(() -> Utils.withLines(fmtDay, handleLines));

    printf("day %s, part %d (%dms): %s%n", fmtDay, part, solution.l(), solution.e());
}

// https://mail.openjdk.org/pipermail/amber-dev/2024-December/009101.html
void printf(final String format, final Object... args) {
    System.out.format(format, args);
}
