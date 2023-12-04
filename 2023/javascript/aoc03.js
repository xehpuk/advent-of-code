import {withLines} from './utils.js'

function createHandleLine(regex) {
    return ({partNumbers, symbolPositions}, line) => {
        const foundPartNumbers = []
        const foundSymbolPositions = []

        for (const match of line.matchAll(regex)) {
            if (match[1]) {
                foundPartNumbers.push({
                    n: match[1],
                    i: match.index,
                })
            } else {
                foundSymbolPositions.push(match.index)
            }
        }

        return {
            partNumbers: [...partNumbers, foundPartNumbers],
            symbolPositions: [...symbolPositions, foundSymbolPositions]
        }
    }
}

export async function part1(fileName = '03') {
    const engineSchematic = await withLines(fileName, createHandleLine(/(\d+)|[^.]/g), {
        partNumbers: [],
        symbolPositions: [],
    })

    return engineSchematic.partNumbers
        .flatMap((partNumbers, j) =>
            partNumbers.filter(({n, i}) => engineSchematic.symbolPositions[j].includes(i - 1) ||
                engineSchematic.symbolPositions[j].includes(i + n.length) ||
                engineSchematic.symbolPositions[j - 1] && Array(n.length + 2).fill().some((_, k) => engineSchematic.symbolPositions[j - 1].includes(i + k - 1)) ||
                engineSchematic.symbolPositions[j + 1] && Array(n.length + 2).fill().some((_, k) => engineSchematic.symbolPositions[j + 1].includes(i + k - 1)))
        )
        .map(({n}) => Number.parseInt(n))
        .reduce((sum, partNumber) => sum + partNumber, 0)
}

export async function part2(fileName = '03') {
    const engineSchematic = await withLines(fileName, createHandleLine(/(\d+)|\*/g), {
        partNumbers: [],
        symbolPositions: [],
    })

    return engineSchematic.symbolPositions
        .flatMap((symbolPositions, j) =>
            symbolPositions.map(symbolPosition => {
                const adjacentPartNumbers = [
                    ...engineSchematic.partNumbers[j].filter(({n, i}) => i + n.length === symbolPosition || i === symbolPosition + 1),
                    ...(engineSchematic.partNumbers[j - 1] ?? []).filter(({n, i}) => i <= symbolPosition + 1 && symbolPosition - 1 < i + n.length),
                    ...(engineSchematic.partNumbers[j + 1] ?? []).filter(({n, i}) => i <= symbolPosition + 1 && symbolPosition - 1 < i + n.length),
                ]
                return adjacentPartNumbers.length === 2
                    ? adjacentPartNumbers[0].n * adjacentPartNumbers[1].n
                    : 0
            })
        )
        .reduce((sum, gearRatio) => sum + gearRatio, 0)
}
