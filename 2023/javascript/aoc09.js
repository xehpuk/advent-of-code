import {withLines} from './utils.js'

function parseLine(line) {
    return line.match(/-?\d+/g).map(n => +n)
}

function calcSum(numbers) {
    let sum = 0
    while (numbers.some(n => n !== 0)) {
        for (let i = 0; i < numbers.length - 1; i++) {
            numbers[i] = numbers[i + 1] - numbers[i]
        }
        sum += numbers.pop()
    }
    return sum
}

export function part1(fileName = '09') {
    return withLines(fileName, (sum, line) => sum + calcSum(parseLine(line)), 0)
}

export function part2(fileName = '09') {
    return withLines(fileName, (sum, line) => sum + calcSum(parseLine(line).toReversed()), 0)
}
