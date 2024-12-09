package de.xehpuk.adventofcode;

import de.xehpuk.adventofcode.Utils.II;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.regex.Pattern;
import java.util.stream.Stream;

public class Day09 {
    private static final Pattern DIGIT = Pattern.compile("\\d");

    public static long part1(final Stream<String> lines) {
        final int[] ids = parseLines(lines);
        for (int i = 0, j = ids.length - 1; i < j; i++) {
            if (ids[i] == -1) {
                while (ids[j] == -1) {
                    j--;
                }
                ids[i] = ids[j];
                ids[j--] = -1;
            }
        }
        long sum = 0;
        for (int i = 0; i < ids.length && ids[i] != -1; i++) {
            sum += i * ids[i];
        }
        return sum;
    }

    public static long part2(final Stream<String> lines) {
        final var ids = parseLines2(lines);

        for (int j = ids.size() - 1; j >= 0; j--) {
            final var item = ids.get(j);
            if (item.l() != -1) {
                for (int i = 0; i < j; i++) {
                    final var gap = ids.get(i);
                    if (gap.l() == -1 && gap.r() >= item.r()) {
                        ids.set(j, new II(-1, item.r()));
                        ids.set(i, item);
                        if (gap.r() > item.r()) {
                            ids.add(i + 1, new II(gap.l(), gap.r() - item.r()));
                            j++;
                        }
                        break;
                    }
                }
            }
        }

        long sum = 0;
        int i = 0;
        for (II id : ids) {
            if (id.l() == -1) {
                i += id.r();
                continue;
            }
            for (int j = 0; j < id.r(); j++, i++) {
                sum += i * id.l();
            }
        }
        return sum;
    }

    private static int[] parseLines(final Stream<String> lines) {
        final var line = lines.findAny().get();
        final var result = new int[line.length() * 9];
        Arrays.fill(result, -1);
        final var matcher = DIGIT.matcher(line);

        int id = 0;
        boolean file = true;
        int i = 0;
        while (matcher.find()) {
            final var length = Integer.parseInt(matcher.group());
            if (file) {
                for (int j = 0; j < length; j++, i++) {
                    result[i] = id;
                }
                id++;
            } else {
                i += length;
            }
            file = !file;
        }

        return Arrays.copyOf(result, i);
    }

    private static List<II> parseLines2(final Stream<String> lines) {
        final var line = lines.findAny().get();
        final var result = new II[line.length()];
        final var matcher = DIGIT.matcher(line);

        int id = 0;
        boolean file = true;
        int i = 0;
        while (matcher.find()) {
            final var length = Integer.parseInt(matcher.group());
            result[i++] = new II(file ? id++ : -1, length);
            file = !file;
        }

        return new ArrayList<>(Arrays.asList(result));
    }
}
