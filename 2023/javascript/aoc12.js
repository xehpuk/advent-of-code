import {withLines} from './utils.js'

const conditions = ['.', '#']

function parseLine(line) {
    const [row, contiguous] = line.split(' ')
    return {
        row,
        contiguous: contiguous.split(',').map(n => +n),
    }
}

export function part1(fileName = '12') {
    return withLines(fileName, (sum, line) => {
        const conditionRecord = parseLine(line)
        const {row, contiguous} = conditionRecord
        const unknownIndices = Array.from(row.matchAll(/(\?)/g), m => m.index)
        const rowArray = Array.from(row)
        const replacementCount = 2 ** unknownIndices.length
        const regex = new RegExp(`^\\.*${contiguous.map(c => `#{${c}}`).join('\\.+')}\\.*$`)
        for (let i = 0; i < replacementCount; i++) {
            for (let j = 0; j < unknownIndices.length; j++) {
                rowArray[unknownIndices[j]] = conditions[replacementCount - i >> j & 1]
            }
            if (regex.test(rowArray.join(''))) {
                sum++
            }
        }
        return sum
    }, 0)
}

export function part2(fileName = '12') {
    return withLines(fileName, (lines, line) => [...lines, line], [])
}
