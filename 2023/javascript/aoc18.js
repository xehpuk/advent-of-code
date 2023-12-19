import {withLines} from './utils.js'

const conversion = {
    R: {
        dx: 1,
        dy: 0,
    },
    D: {
        dx: 0,
        dy: 1,
    },
    L: {
        dx: -1,
        dy: 0,
    },
    U: {
        dx: 0,
        dy: -1,
    },
}

function stringifyHole(x, y) {
    return `${x},${y}`
}

function solve(fileName, parseLine) {
    return withLines(fileName, (digPlan, line) => {
        const {direction, length} = parseLine(line)
        const offset = conversion[direction]
        const x = digPlan.x + offset.dx * length
        const y = digPlan.y + offset.dy * length
        return {
            directions: [...digPlan.directions, ...Array(+length).fill(offset)],
            x,
            y,
            xMin: Math.min(digPlan.xMin, x),
            yMin: Math.min(digPlan.yMin, y),
            xMax: Math.max(digPlan.xMax, x),
            yMax: Math.max(digPlan.yMax, y),
        }
    }, {
        directions: [],
        x: 0,
        y: 0,
        xMin: 0,
        yMin: 0,
        xMax: 0,
        yMax: 0,
    }).then(digPlan => {
        let hole = {
            x: -digPlan.xMin,
            y: -digPlan.yMin,
        }
        const holes = []
        holes[hole.y] = []
        holes[hole.y][hole.x] = 1
        for (const direction of digPlan.directions) {
            hole = {
                x: hole.x + direction.dx,
                y: hole.y + direction.dy,
            }
            holes[hole.y] ??= []
            holes[hole.y][hole.x] = 1
        }
        const exteriorCubes = new Set()
        const checkedCubes = new Set()
        const xLength = digPlan.xMax - digPlan.xMin + 1
        const yLength = digPlan.yMax - digPlan.yMin + 1

        function isExterior(x, y) {
            if (x < 0 || x >= xLength || y < 0 || y >= yLength) {
                return true
            }
            if (holes[y][x]) {
                return false
            }
            const stringified = stringifyHole(x, y)
            if (exteriorCubes.has(stringified)) {
                return true
            }
            if (checkedCubes.has(stringified)) {
                return false
            }
            checkedCubes.add(stringified)
            if (isExterior(x + 1, y) |
                isExterior(x, y + 1) |
                isExterior(x - 1, y) |
                isExterior(x, y - 1)) {
                exteriorCubes.add(stringified)
                return true
            }
            return false
        }

        for (let y = 0; y < yLength; y++) {
            if (isExterior(0, y)) {
                holes[y][0] = 0
            }
            if (isExterior(xLength - 1, y)) {
                holes[y][xLength - 1] = 0
            }
        }
        for (let x = 0; x < xLength; x++) {
            if (isExterior(x, 0)) {
                holes[0][x] = 0
            }
            if (isExterior(x, yLength - 1)) {
                holes[yLength - 1][x] = 0
            }
        }
        return xLength * yLength - checkedCubes.size
    })
}

export function part1(fileName = '18') {
    return solve(fileName, line => {
        const [, direction, length] = line.match(/([RDLU]) (\d+)/)
        return {
            direction,
            length,
        }
    })
}

export function part2(fileName = '18') {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
}
