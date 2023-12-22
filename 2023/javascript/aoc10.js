import {withLines} from './utils.js'

const tileDirections = {
    '-': ['E', 'W'],
    'J': ['W', 'N'],
    '7': ['S', 'W'],
    '|': ['S', 'N'],
    'L': ['E', 'N'],
    'F': ['E', 'S'],
}

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
        const startIndex = line.indexOf('S')
        return {
            start: grid.start || startIndex !== -1 && [index, startIndex],
            tiles: [
                ...grid.tiles,
                line,
            ],
        }
    }, {tiles: []})
}

/**
 * @returns {boolean} whether the starting position connects north
 */
function walkLoop(start, tiles, handlePipe) {
    function findFirstPipe() {
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
    }

    let pipe = findFirstPipe()
    const startConnectsNorth = pipe.direction === 'N'
    while (pipe.y !== start[0] || pipe.x !== start[1]) {
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
    return startConnectsNorth || pipe.direction === opposites['N']
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
            const tilesConnectingNorth = new Set(
                Object.entries(tileDirections)
                    .filter(([, directions]) => directions.includes('N'))
                    .map(([tile]) => tile),
            )

            const loop = []

            if (walkLoop(start, tiles, pipe => {
                loop[pipe.y] ??= []
                loop[pipe.y][pipe.x] = pipe.label
            })) {
                tilesConnectingNorth.add('S')
            }

            let enclosed = 0
            for (let y = 0; y < tiles.length; y++) {
                const row = loop[y]
                if (row === undefined) {
                    continue
                }
                let interior = 0
                for (let x = 0; x < tiles[y].length - 1; x++) {
                    const label = row[x]
                    if (label !== undefined) {
                        interior ^= tilesConnectingNorth.has(label)
                    } else {
                        enclosed += interior
                    }
                }
            }
            return enclosed
        })
}
