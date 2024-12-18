package de.xehpuk.adventofcode;

import java.util.List;
import java.util.StringJoiner;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Gatherers;
import java.util.stream.Stream;

public class Day17 {
    private static final Pattern DIGIT = Pattern.compile("\\d");

    public static String part1(final Stream<String> lines) {
        Program program = parseProgram(lines);
        int a = program.registerA();
        int b = program.registerB();
        int c = program.registerC();
        StringJoiner result = new StringJoiner(",");
        int instructionPointer = 0;
        while (instructionPointer < program.instructions().size()) {
            Instruction instruction = program.instructions().get(instructionPointer);
            int comboOperand = switch (instruction.operand()) {
                case 4 -> a;
                case 5 -> b;
                case 6 -> c;
                case int o -> o;
            };
            switch (instruction.operator()) {
                case ADV -> a = Math.toIntExact(a / Utils.pow(2, comboOperand));
                case BXL -> b = b ^ instruction.operand();
                case BST -> b = comboOperand % 8;
                case JNZ -> {
                    if (a != 0) {
                        instructionPointer = instruction.operand() / 2;
                        continue;
                    }
                }
                case BXC -> b = b ^ c;
                case OUT -> result.add(String.valueOf(comboOperand % 8));
                case BDV -> b = Math.toIntExact(a / Utils.pow(2, comboOperand));
                case CDV -> c = Math.toIntExact(a / Utils.pow(2, comboOperand));
            }
            instructionPointer++;
        }
        return result.toString();
    }

    public static long part2(final Stream<String> lines) {
        return 0L;
    }

    private static Program parseProgram(final Stream<String> lines) {
        final var iterator = lines.iterator();
        final String registerALine = iterator.next();
        final int registerA = Integer.parseInt(registerALine.substring(registerALine.lastIndexOf(' ') + 1));
        final String registerBLine = iterator.next();
        final int registerB = Integer.parseInt(registerBLine.substring(registerBLine.lastIndexOf(' ') + 1));
        final String registerCLine = iterator.next();
        final int registerC = Integer.parseInt(registerCLine.substring(registerCLine.lastIndexOf(' ') + 1));
        iterator.next();
        final String programLine = iterator.next();
        final List<Instruction> instructions = DIGIT.matcher(programLine.substring(programLine.lastIndexOf(' ') + 1)).results()
                .map(MatchResult::group)
                .map(Integer::parseInt)
                .gather(Gatherers.windowFixed(2))
                .map(ints -> new Instruction(Op.values()[ints.getFirst()], ints.getLast()))
                .toList();
        return new Program(registerA, registerB, registerC, instructions);
    }

    record Program(
            int registerA,
            int registerB,
            int registerC,
            List<Instruction> instructions
    ) {
    }

    record Instruction(Op operator, int operand) {
    }

    enum Op {
        ADV,
        BXL,
        BST,
        JNZ,
        BXC,
        OUT,
        BDV,
        CDV,
    }
}
