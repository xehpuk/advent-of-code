import de.xehpuk.*;

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
}

void solve(final int day, final int part, final Function<Stream<String>, ?> handleLines) {
    final var fmtDay = "%02d".formatted(day);
    final var solution = Utils.withLines(fmtDay, handleLines);

    printf("day %s, part %d: %s%n", fmtDay, part, solution);
}

// https://mail.openjdk.org/pipermail/amber-dev/2024-December/009101.html
void printf(final String format, final Object... args) {
    System.out.format(format, args);
}
