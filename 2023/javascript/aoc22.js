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
        name: String.fromCharCode(65 + index),
    }
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
                        if (brick.x.min <= brickBelow.x.max &&
                            brickBelow.x.min <= brick.x.max &&
                            brick.y.min <= brickBelow.y.max &&
                            brickBelow.y.min <= brick.y.max) {
                            brick.below ??= new Set()
                            brick.below.add(brickBelow.name)
                            airborne = false
                        }
                    }
                    if (airborne) {
                        bricks[zn].splice(i, 1)
                        brick.z.min--
                        brick.z.max--
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
                    if (brickAbove.below.has(brick.name) && brickAbove.below.size === 1) {
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
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
}
