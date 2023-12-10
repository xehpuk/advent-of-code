import {withLines} from './utils.js'

function parseLine(line) {
    return line.match(/-?\d+/g).map(n => +n)
}

export function part1(fileName = '09') {
    return withLines(fileName, (sum, line) => {
        const numbers = parseLine(line)
        while (numbers.some(n => n !== 0)) {
            for (let i = 0; i < numbers.length - 1; i++) {
                numbers[i] = numbers[i + 1] - numbers[i]
            }
            sum += numbers.pop()
        }
        return sum
    }, 0)
}

export function part2(fileName = '09') {
    return withLines(fileName, (lines, line) => lines.concat(line), [])
}
