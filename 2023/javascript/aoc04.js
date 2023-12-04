import {withLines} from './utils.js'

function parseLine(line) {
    const [, scratchcard] = line.split(': ')
    const [winningNumbers, actualNumbers] = scratchcard
        .split(' | ')
        .map(numbers => numbers.match(/\d+/g)
            .map(n => +n)
        )
    return {
        winningNumbers,
        actualNumbers,
    }
}

function calculateNumberOfMatchingNumbers(winningNumbers, actualNumbers) {
    const setOfWinningNumbers = new Set(winningNumbers)
    return actualNumbers.filter(actualNumber => setOfWinningNumbers.has(actualNumber)).length
}

export function part1(fileName = '04') {
    function calculatePoints(winningNumbers, actualNumbers) {
        const numberOfMatchingNumbers = calculateNumberOfMatchingNumbers(winningNumbers, actualNumbers)
        return numberOfMatchingNumbers && 2 ** (numberOfMatchingNumbers - 1)
    }

    return withLines(fileName, (sum, line) => {
        const {
            winningNumbers,
            actualNumbers,
        } = parseLine(line)
        return sum + calculatePoints(winningNumbers, actualNumbers)
    }, 0)
}

export function part2(fileName = '04') {
    return withLines(fileName, ({totalScratchcards, scratchcards}, line) => {
        const {
            winningNumbers,
            actualNumbers,
        } = parseLine(line)
        const numberOfMatchingNumbers = calculateNumberOfMatchingNumbers(winningNumbers, actualNumbers)
        const [current = 1, ...remaining] = scratchcards
        return {
            totalScratchcards: totalScratchcards + current,
            scratchcards: Array(Math.max(remaining.length, numberOfMatchingNumbers))
                .fill()
                .map((_, i) => i < numberOfMatchingNumbers
                    ? (remaining[i] ?? 1) + current
                    : remaining[i],
                ),
        }
    }, {
        totalScratchcards: 0,
        scratchcards: [],
    }).then(({totalScratchcards}) => totalScratchcards)
}
