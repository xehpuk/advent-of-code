import {withLines} from './utils.js';

export async function part1(fileName = '03') {
    const engineSchematic = await withLines(fileName, ({partNumbers, symbolPositions}, line) => {
        const foundPartNumbers = []
        const foundSymbolPositions = []

        for (const match of line.matchAll(/(\d+)|([^.])/g)) {
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
    }, {
        partNumbers: [],
        symbolPositions: [],
    })

    return engineSchematic.partNumbers.flatMap((partNumbers, j) =>
        partNumbers.filter(({n, i}) => engineSchematic.symbolPositions[j].includes(i - 1) ||
            engineSchematic.symbolPositions[j].includes(i + n.length) ||
            engineSchematic.symbolPositions[j - 1] && Array(n.length + 2).fill().some((_, k) => engineSchematic.symbolPositions[j - 1].includes(i + k - 1)) ||
            engineSchematic.symbolPositions[j + 1] && Array(n.length + 2).fill().some((_, k) => engineSchematic.symbolPositions[j + 1].includes(i + k - 1))))
        .map(({n}) => Number.parseInt(n))
        .reduce((sum, partNumber) => sum + partNumber, 0)
}

export function part2(fileName = '03') {
    throw new Error('TODO')
}
