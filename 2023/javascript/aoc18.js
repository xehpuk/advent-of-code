import {withLines} from './utils.js'

function stringifyCube(x, y) {
    return `${x},${y}`
}

function solve(fileName, parseLine) {
    return withLines(fileName, (digPlan, line) => {
        const {offset, length} = parseLine(line)
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
        const cubesToCheck = []
        const checkedCubes = new Set()
        const xLength = digPlan.xMax - digPlan.xMin + 1
        const yLength = digPlan.yMax - digPlan.yMin + 1

        function push(x, y) {
            if (checkedCubes.has(stringifyCube(x, y))) {
                return
            }
            cubesToCheck.push({x, y})
        }

        function checkCube(x, y) {
            if (x < 0 || x >= xLength || y < 0 || y >= yLength || holes[y][x]) {
                return
            }
            checkedCubes.add(stringifyCube(x, y))
            push(x + 1, y)
            push(x, y + 1)
            push(x - 1, y)
            push(x, y - 1)
        }

        for (let y = 0; y < yLength; y++) {
            cubesToCheck.push({x: 0, y})
            cubesToCheck.push({x: xLength - 1, y})
        }
        for (let x = 1; x < xLength - 1; x++) {
            cubesToCheck.push({x, y: 0})
            cubesToCheck.push({x, y: yLength - 1})
        }
        while (cubesToCheck.length > 0) {
            const cube = cubesToCheck.pop()
            checkCube(cube.x, cube.y)
        }
        return xLength * yLength - checkedCubes.size
    })
}

export function part1(fileName = '18') {
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

    return solve(fileName, line => {
        const [, direction, length] = line.match(/([RDLU]) (\d+)/)
        return {
            offset: conversion[direction],
            length,
        }
    })
}

export function part2(fileName = '18') {
    return solve(fileName, line => {
        const conversion = [
            {
                dx: 1,
                dy: 0,
            },
            {
                dx: 0,
                dy: 1,
            },
            {
                dx: -1,
                dy: 0,
            },
            {
                dx: 0,
                dy: -1,
            },
        ]

        const [, length, direction] = line.match(/#(.{5})(.)/)
        return {
            offset: conversion[direction],
            length: Number.parseInt(length, 16),
        }
    })
}
