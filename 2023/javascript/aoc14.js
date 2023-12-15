import {withLines} from './utils.js'

const ROUND = 1
const CUBE = -1
const EMPTY = 0

function parseRow(row) {
    return Array.from(row).map(stone => {
        switch (stone) {
            case 'O': return ROUND
            case '#': return CUBE
            default: return EMPTY
        }
    })
}

function tiltNorth(platform) {
    for (let i = 1; i < platform.length; i++) {
        for (let j = i; j > 0; j--) {
            const currentRow = platform[j]
            const northernRow = platform[j - 1]
            let moved = false
            for (let k = 0; k < currentRow.length; k++) {
                if (currentRow[k] === ROUND && northernRow[k] === EMPTY) {
                    northernRow[k] = currentRow[k]
                    currentRow[k] = EMPTY
                    moved = true
                }
            }
            if (!moved) {
                break
            }
        }
    }
}

function calcTotalLoad(platform) {
    let totalLoad = 0
    for (let i = 0; i < platform.length; i++) {
        const load = platform.length - i
        for (const rock of platform[i]) {
            if (rock === ROUND) {
                totalLoad += load
            }
        }
    }
    return totalLoad
}

export function part1(fileName = '14') {
    return withLines(fileName, (platform, row) => [...platform, parseRow(row)], [])
        .then(platform => {
            tiltNorth(platform)

            return calcTotalLoad(platform)
        })
}

export function part2(fileName = '14') {
    return withLines(fileName, (lines, line) => [...lines, line], [])
}
