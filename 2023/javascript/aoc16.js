import {withLines} from './utils.js'

function stringifyTile(tile) {
    return `${tile.x},${tile.y}`
}

function stringifyBeam(beam) {
    return `${beam.x},${beam.y},${beam.dx},${beam.dy}`
}

function energize(contraption, initialBeam) {
    const energizedTiles = new Set()
    const beamTrail = new Set()
    const beams = [initialBeam]

    while (beams.length > 0) {
        const beam = beams[0]
        beam.x += beam.dx
        beam.y += beam.dy
        if (beam.y >= 0 && beam.y < contraption.length &&
            beam.x >= 0 && beam.x < contraption[beam.y].length) {
            const beamId = stringifyBeam(beam)
            if (!beamTrail.has(beamId)) {
                beamTrail.add(beamId)
                energizedTiles.add(stringifyTile(beam))

                switch (contraption[beam.y][beam.x]) {
                    case '|':
                        if (beam.dx !== 0) {
                            beams.push({
                                ...beam,
                                dx: 0,
                                dy: 1,
                            })
                            beam.dx = 0
                            beam.dy = -1
                        }
                        break
                    case '-':
                        if (beam.dy !== 0) {
                            beams.push({
                                ...beam,
                                dx: 1,
                                dy: 0,
                            })
                            beam.dx = -1
                            beam.dy = 0
                        }
                        break
                    case '\\':
                        if (beam.dx !== 0) {
                            beam.dy = beam.dx
                            beam.dx = 0
                        } else {
                            beam.dx = beam.dy
                            beam.dy = 0
                        }
                        break
                    case '/':
                        if (beam.dx !== 0) {
                            beam.dy = -beam.dx
                            beam.dx = 0
                        } else {
                            beam.dx = -beam.dy
                            beam.dy = 0
                        }
                        break
                }
                continue
            }
        }
        beams.splice(0, 1)
    }

    return energizedTiles.size
}

export function part1(fileName = '16') {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
        .then(contraption =>
            energize(contraption, {
                x: -1,
                y: 0,
                dx: 1,
                dy: 0,
            }))
}

export function part2(fileName = '16') {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
        .then(contraption => {
            let maxEnergy = 0
            for (let y = 0; y < contraption.length; y++) {
                maxEnergy = Math.max(maxEnergy, energize(contraption, {
                    x: -1,
                    y,
                    dx: 1,
                    dy: 0,
                }))
                maxEnergy = Math.max(maxEnergy, energize(contraption, {
                    x: contraption[y].length,
                    y,
                    dx: -1,
                    dy: 0,
                }))
            }
            for (let x = 0; x < contraption[0].length; x++) {
                maxEnergy = Math.max(maxEnergy, energize(contraption, {
                    x,
                    y: -1,
                    dx: 0,
                    dy: 1,
                }))
                maxEnergy = Math.max(maxEnergy, energize(contraption, {
                    x,
                    y: contraption.length,
                    dx: 0,
                    dy: -1,
                }))
            }
            return maxEnergy
        })
}
