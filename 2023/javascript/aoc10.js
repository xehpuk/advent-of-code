import {withLines} from './utils.js'

const startLabel = 'S'

const tileDirections = {
    '-': ['E', 'W'],
    'J': ['W', 'N'],
    '7': ['S', 'W'],
    '|': ['S', 'N'],
    'L': ['E', 'N'],
    'F': ['E', 'S'],
}

const tilesPointingNorth = new Set(
    Object.entries(tileDirections)
        .filter(([, directions]) => directions.includes('N'))
        .map(([tile]) => tile),
)

const directions = {
    E: ([lat, long]) => [lat, long + 1],
    S: ([lat, long]) => [lat + 1, long],
    W: ([lat, long]) => [lat, long - 1],
    N: ([lat, long]) => [lat - 1, long],
}

const opposites = {
    E: 'W',
    S: 'N',
    W: 'E',
    N: 'S',
}

export function part1(fileName = '10') {
    return withLines(fileName, (grid, line, index) => {
        const startIndex = line.indexOf(startLabel)
        return {
            start: grid.start || startIndex !== -1 && [index, startIndex],
            tiles: [
                ...grid.tiles,
                line,
            ],
        }
    }, {tiles: []}).then(({start, tiles}) => {
        let tile = (() => {
            for (const [direction, move] of Object.entries(directions)) {
                const [nextY, nextX] = move(start)
                const label = tiles[nextY][nextX]
                if (tileDirections[label].includes(opposites[direction])) {
                    return {
                        direction,
                        nextY,
                        nextX,
                        label,
                    }
                }
            }
        })()
        let i = 1
        while (tile.label !== startLabel) {
            const direction = tileDirections[tile.label].find(direction => direction !== opposites[tile.direction])
            const [nextY, nextX] = directions[direction]([tile.nextY, tile.nextX])
            const label = tiles[nextY][nextX]
            tile = {
                direction,
                nextY,
                nextX,
                label,
            }
            i++
        }
        return i / 2
    })
}

export function part2(fileName = '10') {
    return withLines(fileName, (grid, line, index) => {
        const startIndex = line.indexOf(startLabel)
        return {
            start: grid.start || startIndex !== -1 && [index, startIndex],
            tiles: [
                ...grid.tiles,
                line,
            ],
        }
    }, {tiles: []}).then(({start, tiles}) => {
        const loop = []
        let tile = (() => {
            for (const [direction, move] of Object.entries(directions)) {
                const [nextY, nextX] = move(start)
                const label = tiles[nextY][nextX]
                if (tileDirections[label].includes(opposites[direction])) {
                    loop[nextY] = []
                    loop[nextY][nextX] = tilesPointingNorth.has(label)

                    return {
                        direction,
                        nextY,
                        nextX,
                        label,
                    }
                }
            }
        })()
        while (tile.label !== startLabel) {
            const direction = tileDirections[tile.label].find(direction => direction !== opposites[tile.direction])
            const [nextY, nextX] = directions[direction]([tile.nextY, tile.nextX])
            const label = tiles[nextY][nextX]

            loop[nextY] ??= []
            loop[nextY][nextX] = tilesPointingNorth.has(label)

            tile = {
                direction,
                nextY,
                nextX,
                label,
            }
        }

        let interior = 0
        for (let y = 0; y < tiles.length; y++) {
            const row = loop[y]
            if (row === undefined) {
                continue
            }
            let numberOfPipes = 0
            for (let x = 0; x < tiles[y].length - 1; x++) {
                const pipe = row[x]
                if (pipe !== undefined) {
                    numberOfPipes ^= pipe
                } else {
                    interior += numberOfPipes
                }
            }
        }
        return interior
    })
}
