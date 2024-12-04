package de.xehpuk;

import java.util.List;
import java.util.stream.Stream;

public class Day04 {
    public static long part1(final Stream<String> lines) {
        final var list = lines.toList();

        var sum = 0;
        for (int y = 0; y < list.size(); y++) {
            for (int x = 0; x < list.get(y).length(); x++) {
                for (int i = -1; i <= 1; i++) {
                    for (int j = -1; j <= 1; j++) {
                        if (i != 0 || j != 0) { // optional check
                            if (find(x, y, i, j, list)) {
                                sum++;
                            }
                        }
                    }
                }
            }
        }
        return sum;
    }

    private static boolean find(final int x0, final int y0, final int dx, final int dy, final List<String> list) {
        var x = x0;
        var y = y0;
        for (final var c : "XMAS".toCharArray()) {
            if (y < 0 || y >= list.size() || x < 0 || x >= list.get(y).length() || c != list.get(y).charAt(x)) {
                return false;
            }
            x += dx;
            y += dy;
        }
        return true;
    }

    public static long part2(final Stream<String> lines) {
        final var list = lines.toList();

        var sum = 0;
        for (int y = 1; y < list.size() - 1; y++) {
            for (int x = 1; x < list.get(y).length() - 1; x++) {
                if (find2(x, y, list)) {
                    sum++;
                }
            }
        }
        return sum;
    }

    private static boolean find2(final int x, final int y, final List<String> list) {
        if (list.get(y).charAt(x) != 'A') {
            return false;
        }

        final var c1 = list.get(y - 1).charAt(x - 1);
        final var c2 = list.get(y - 1).charAt(x + 1);
        final var c3 = list.get(y + 1).charAt(x - 1);
        final var c4 = list.get(y + 1).charAt(x + 1);

        return (c1 == 'M' && c4 == 'S' || c1 == 'S' && c4 == 'M') &&
               (c2 == 'M' && c3 == 'S' || c2 == 'S' && c3 == 'M');
    }
}
