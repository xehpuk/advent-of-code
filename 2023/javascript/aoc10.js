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
        const loop = [tile]
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
            loop.push(tile)
        }

        const tilesPointingNorth = new Set(
            Object.entries(tileDirections)
                .filter(([, directions]) => directions.includes('N'))
                .map(([tile]) => tile),
        )

        let interior = 0
        for (let y = 0; y < tiles.length; y++) {
            let numberOfPipes = 0
            for (let x = 0; x < tiles[y].length; x++) {
                const pipe = loop.find(({nextY, nextX}) => nextY === y && nextX === x)
                if (pipe) {
                    if (tilesPointingNorth.has(pipe.label)) {
                        numberOfPipes ^= 1
                    }
                } else {
                    interior += numberOfPipes
                }
            }
        }
        return interior
    })
}
