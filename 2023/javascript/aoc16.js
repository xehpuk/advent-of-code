import {withLines} from './utils.js'

function equals(tile1, tile2) {
    return tile1.x === tile2.x &&
        tile1.y === tile2.y &&
        tile1.dx === tile2.dx &&
        tile1.dy === tile2.dy
}

function energize(contraption, initialBeam) {
    const energizedTiles = []
    const beams = [initialBeam]

    while (beams.length > 0) {
        const beam = beams[0]
        beam.x += beam.dx
        beam.y += beam.dy
        if (beam.y >= 0 && beam.y < contraption.length &&
            beam.x >= 0 && beam.x < contraption[beam.y].length &&
            !energizedTiles.some(tile => equals(tile, beam))) {
            energizedTiles.push({...beam})

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
        } else {
            beams.splice(0, 1)
        }
    }

    return energizedTiles.reduce((uniqueTiles, tile) => uniqueTiles.add(`${tile.x},${tile.y}`), new Set()).size
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
}
