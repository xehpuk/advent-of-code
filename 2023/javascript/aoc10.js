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
    E: ([y, x]) => [y, x + 1],
    S: ([y, x]) => [y + 1, x],
    W: ([y, x]) => [y, x - 1],
    N: ([y, x]) => [y - 1, x],
}

const opposites = {
    E: 'W',
    S: 'N',
    W: 'E',
    N: 'S',
}

function parseGrid(fileName) {
    return withLines(fileName, (grid, line, index) => {
        const startIndex = line.indexOf(startLabel)
        return {
            start: grid.start || startIndex !== -1 && [index, startIndex],
            tiles: [
                ...grid.tiles,
                line,
            ],
        }
    }, {tiles: []})
}

function walkLoop(start, tiles, handlePipe) {
    let pipe = (() => {
        for (const [direction, move] of Object.entries(directions)) {
            const [y, x] = move(start)
            const label = tiles[y][x]
            if (tileDirections[label].includes(opposites[direction])) {
                const pipe = {
                    direction,
                    y,
                    x,
                    label,
                }
                handlePipe(pipe)
                return pipe
            }
        }
    })()
    while (pipe.label !== startLabel) {
        const direction = tileDirections[pipe.label].find(direction => direction !== opposites[pipe.direction])
        const [y, x] = directions[direction]([pipe.y, pipe.x])
        const label = tiles[y][x]

        pipe = {
            direction,
            y,
            x,
            label,
        }
        handlePipe(pipe)
    }
}

export function part1(fileName = '10') {
    return parseGrid(fileName)
        .then(({start, tiles}) => {
            let i = 0

            walkLoop(start, tiles, _ => {
                i++
            })

            return i / 2
        })
}

export function part2(fileName = '10') {
    return parseGrid(fileName)
        .then(({start, tiles}) => {
            const loop = []

            walkLoop(start, tiles, pipe => {
                loop[pipe.y] ??= []
                loop[pipe.y][pipe.x] = tilesPointingNorth.has(pipe.label)
            })

            let enclosed = 0
            for (let y = 0; y < tiles.length; y++) {
                const row = loop[y]
                if (row === undefined) {
                    continue
                }
                let interior = 0
                for (let x = 0; x < tiles[y].length - 1; x++) {
                    const north = row[x]
                    if (north !== undefined) {
                        interior ^= north
                    } else {
                        enclosed += interior
                    }
                }
            }
            return enclosed
        })
}
