import { open } from 'node:fs/promises'
import { join } from 'node:path'

async function withLines(fileName, handleLine, initialValue) {
    const file = await open(join('..', `${fileName}.txt`))
    try {
        let value = initialValue
        for await (const line of file.readLines()) {
            value = handleLine(value, line)
        }
        return value
    } finally {
        await file.close()
    }
}

function solve(fileName, parseLine) {
    return withLines(fileName, (value, line) => value + calculateCalibrationValue(parseLine(line)), 0)
}

function calculateCalibrationValue(digits) {
    return 10 * digits[0] + digits[digits.length - 1]
}

export function part1(fileName = '01') {
    function parseLine(line) {
        return Array.from(line.matchAll(/\d/g), match => Number.parseInt(match[0]))
    }

    return solve(fileName, parseLine)
}

export function part2(fileName = '01') {
    const digits = {
        one: 1,
        two: 2,
        three: 3,
        four: 4,
        five: 5,
        six: 6,
        seven: 7,
        eight: 8,
        nine: 9,
    }

    const digitRegex = new RegExp(`(?=(${Object.entries(digits).map(entry => entry.join('|')).join('|')}))`, 'g')

    function parseDigit(digit) {
        return digits[digit] ?? Number.parseInt(digit)
    }

    function parseLine(line) {
        return Array.from(line.matchAll(digitRegex), match => parseDigit(match[1]))
    }

    return solve(fileName, parseLine)
}
