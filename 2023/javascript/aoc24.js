import {withLines} from './utils.js'

function parseHailstone(line) {
    const [x, y, z, dx, dy, dz] = line.match(/-?\d+/g).map(n => +n)
    return {x, y, z, dx, dy, dz}
}

/**
 * y = mx + b
 */
function toLinearFunction({x, y, dx, dy}) {
    const m = dy / dx
    const b = y - m * x
    return {m, b, x, dx: Math.sign(dx)}
}

function intersect(f, g) {
    const x = (g.b - f.b) / (f.m - g.m)
    const y = f.m * x + f.b
    return {x, y}
}

export function part1(fileName = '24', min = 200000000000000, max = 400000000000000) {
    return withLines(fileName, (lines, line) => [...lines, parseHailstone(line)], [])
        .then(hailstones => hailstones.map(toLinearFunction))
        .then(linearFunctions => {
            let intersections = 0
            for (let i = 0; i < linearFunctions.length - 1; i++) {
                const f = linearFunctions[i]
                for (let j = i + 1; j < linearFunctions.length; j++) {
                    const g = linearFunctions[j]
                    const intersection = intersect(f, g)
                    if (intersection.x >= min &&
                        intersection.x <= max &&
                        f.dx * intersection.x >= f.x * f.dx &&
                        g.dx * intersection.x >= g.x * g.dx &&
                        intersection.y >= min &&
                        intersection.y <= max)
                    intersections++
                }
            }
            return intersections
        })
}

export function part2(fileName = '24') {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
}
