import {withLines} from './utils.js'

function parseBrick(line, index) {
    const [xMin, yMin, zMin, xMax, yMax, zMax] = line.match(/\d+/g)
    return {
        x: {
            min: +xMin,
            max: +xMax,
        },
        y: {
            min: +yMin,
            max: +yMax,
        },
        z: {
            min: +zMin,
            max: +zMax,
        },
        id: index,
    }
}

function intersect(range1, range2) {
    return range1.min <= range2.max &&
        range2.min <= range1.max
}

function decrement(range) {
    range.min--
    range.max--
}

export function part1(fileName = '22') {
    return withLines(fileName, (bricks, line, index) => {
        const brick = parseBrick(line, index)
        bricks[brick.z.max] ??= []
        bricks[brick.z.max].push(brick)
        return bricks
    }, []).then(bricks => {
        for (let z = 2; z < bricks.length; z++) {
            for (let zn = z; zn > 1; zn--) {
                for (let i = (bricks[zn] ?? []).length - 1; i >= 0; i--) {
                    const brick = bricks[zn][i]
                    if (brick.z.min === 1) {
                        continue
                    }
                    let airborne = true
                    for (const brickBelow of bricks[brick.z.min - 1] ?? []) {
                        if (intersect(brick.x, brickBelow.x) &&
                            intersect(brick.y, brickBelow.y)) {
                            brick.below ??= new Set()
                            brick.below.add(brickBelow.id)
                            airborne = false
                        }
                    }
                    if (airborne) {
                        bricks[zn].splice(i, 1)
                        decrement(brick.z)
                        bricks[zn - 1] ??= []
                        bricks[zn - 1].push(brick)
                    }
                }
            }
        }
        return bricks
    }).then(bricksMax => {
        const bricksMin = []
        for (let z = 1; z < bricksMax.length; z++) {
            for (const brick of bricksMax[z]) {
                bricksMin[brick.z.min] ??= []
                bricksMin[brick.z.min].push(brick)
            }
        }
        return bricksMin
    }).then(bricks => {
        let disintegratables = 0
        for (let z = 1; z < bricks.length; z++) {
            for (const brick of bricks[z] ?? []) {
                let disintegratable = true
                for (const brickAbove of bricks[brick.z.max + 1] ?? []) {
                    if (brickAbove.below.has(brick.id) && brickAbove.below.size === 1) {
                        disintegratable = false
                        break
                    }
                }
                if (disintegratable) {
                    disintegratables++
                }

            }
        }
        return disintegratables
    })
}

export function part2(fileName = '22') {
    return withLines(fileName, (bricks, line, index) => {
        const brick = parseBrick(line, index)
        bricks[brick.z.max] ??= []
        bricks[brick.z.max].push(brick)
        return bricks
    }, []).then(bricks => {
        const graph = []
        for (let z = 2; z < bricks.length; z++) {
            for (let zn = z; zn > 1; zn--) {
                for (let i = (bricks[zn] ?? []).length - 1; i >= 0; i--) {
                    const brick = bricks[zn][i]
                    if (brick.z.min === 1) {
                        continue
                    }
                    let airborne = true
                    for (const brickBelow of bricks[brick.z.min - 1] ?? []) {
                        if (intersect(brick.x, brickBelow.x) &&
                            intersect(brick.y, brickBelow.y)) {
                            graph[brick.id] ??= {}
                            graph[brick.id].below ??= new Set()
                            graph[brick.id].below.add(brickBelow.id)
                            graph[brickBelow.id] ??= {}
                            graph[brickBelow.id].above ??= new Set()
                            graph[brickBelow.id].above.add(brick.id)
                            airborne = false
                        }
                    }
                    if (airborne) {
                        bricks[zn].splice(i, 1)
                        decrement(brick.z)
                        bricks[zn - 1] ??= []
                        bricks[zn - 1].push(brick)
                    }
                }
            }
        }
        return graph
    }).then(graph =>
        graph.map((_, id) => {
            const fallenBricks = new Set([id])
            for (const fallenBrick of fallenBricks) {
                for (const aboveId of graph[fallenBrick].above ?? []) {
                    const {below} = graph[aboveId]
                    if (below.size === 1 || Array.from(below).every(below => fallenBricks.has(below))) {
                        fallenBricks.add(aboveId)
                    }
                }
            }
            return fallenBricks.size - 1
        }).reduce((sum, val) => sum + val, 0))
}
