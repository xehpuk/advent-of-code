import {withLines} from './utils.js'

function calcNote(pattern) {
    const maxRow = pattern.length - 1
    r: for (let row = 0; row < maxRow; row++) {
        const length = Math.min(maxRow - row, row + 1)
        for (let y = 0; y < length; y++) {
            const originalRow = pattern[row - y]
            const mirroredRow = pattern[row + y + 1]
            if (originalRow !== mirroredRow) {
                continue r
            }
        }
        return 100 * (row + 1)
    }

    const maxColumn = pattern[0].length - 1
    c: for (let column = 0; column < maxColumn; column++) {
        const length = Math.min(maxColumn - column, column + 1)
        for (let x = 0; x < length; x++) {
            for (const row of pattern) {
                const originalColumn = row[column - x]
                const mirroredColumn = row[column + x + 1]
                if (originalColumn !== mirroredColumn) {
                    continue c
                }
            }
        }
        return column + 1
    }
}

export function part1(fileName = '13') {
    return withLines(fileName, (patterns, line) => {
        if (line.length === 0) {
            return [
                ...patterns,
                [],
            ]
        }
        const previousPatterns = patterns.slice(0, -1)
        const currentPattern = patterns[patterns.length - 1]
        return [
            ...previousPatterns,
            [...currentPattern, line],
        ]
    }, [[]]).then(patterns => patterns.map(calcNote)
        .reduce((sum, note) => sum + note, 0))
}

export function part2(fileName = '13') {
    return withLines(fileName, (lines, line) => [...lines, line], [])
}
