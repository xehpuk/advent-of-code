import de.xehpuk.*;

import static de.xehpuk.Utils.withLines;

void main() {
    printf("day 01, part 1: %s%n", withLines("01", Day01::part1));
    printf("day 01, part 2: %s%n", withLines("01", Day01::part2));
    printf("day 02, part 1: %s%n", withLines("02", Day02::part1));
    printf("day 02, part 2: %s%n", withLines("02", Day02::part2));
    printf("day 03, part 1: %s%n", withLines("03", Day03::part1));
    printf("day 03, part 2: %s%n", withLines("03", Day03::part2));
    printf("day 04, part 1: %s%n", withLines("04", Day04::part1));
    printf("day 04, part 2: %s%n", withLines("04", Day04::part2));
}

// https://mail.openjdk.org/pipermail/amber-dev/2024-December/009101.html
void printf(final String format, final Object... args) {
    System.out.format(format, args);
}
