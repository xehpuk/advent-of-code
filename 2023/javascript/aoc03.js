import {withLines} from './utils.js';

export async function part1(fileName = '03') {
    const engineSchematic = await withLines(fileName, ({partNumbers, symbolPositions}, line) => {
        const foundPartNumbers = []
        const foundSymbolPositions = []

        for (const match of line.matchAll(/(\d+)|[^.]/g)) {
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
            engineSchematic.symbolPositions[j + 1] && Array(n.length + 2).fill().some((_, k) => engineSchematic.symbolPositions[j + 1].includes(i + k - 1)))
    )
        .map(({n}) => Number.parseInt(n))
        .reduce((sum, partNumber) => sum + partNumber, 0)
}

export async function part2(fileName = '03') {
    const engineSchematic = await withLines(fileName, ({potentialPartNumbers, potentialGearPositions}, line) => {
        const foundPotentialPartNumbers = []
        const foundPotentialGearPositions = []

        for (const match of line.matchAll(/(\d+)|\*/g)) {
            if (match[1]) {
                foundPotentialPartNumbers.push({
                    n: match[1],
                    i: match.index,
                })
            } else {
                foundPotentialGearPositions.push(match.index)
            }
        }

        return {
            potentialPartNumbers: [...potentialPartNumbers, foundPotentialPartNumbers],
            potentialGearPositions: [...potentialGearPositions, foundPotentialGearPositions]
        }
    }, {
        potentialPartNumbers: [],
        potentialGearPositions: [],
    })

    return engineSchematic.potentialGearPositions.flatMap((potentialGearPositions, j) => {
        return potentialGearPositions.map(potentialGearPosition => {
            const adjacentPartNumbers = [
                ...engineSchematic.potentialPartNumbers[j].filter(({n, i}) => i + n.length === potentialGearPosition || i === potentialGearPosition + 1),
                ...(engineSchematic.potentialPartNumbers[j - 1] ?? []).filter(({n, i}) => i <= potentialGearPosition + 1 && potentialGearPosition - 1 < i + n.length),
                ...(engineSchematic.potentialPartNumbers[j + 1] ?? []).filter(({n, i}) => i <= potentialGearPosition + 1 && potentialGearPosition - 1 < i + n.length),
            ]
            return adjacentPartNumbers.length === 2
                ? adjacentPartNumbers[0].n * adjacentPartNumbers[1].n
                : 0
        })
    })
        .reduce((sum, gearRatio) => sum + gearRatio, 0)
}
