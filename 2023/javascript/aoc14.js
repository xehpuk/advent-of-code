import {withLines} from './utils.js'

const ROUND = 1
const CUBE = -1
const EMPTY = 0

function parseRow(row) {
    return Array.from(row).map(rock => {
        switch (rock) {
            case 'O':
                return ROUND
            case '#':
                return CUBE
            default:
                return EMPTY
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

function tiltWest(platform) {
    for (const row of platform) {
        for (let i = 1; i < row.length; i++) {
            for (let j = i; j > 0; j--) {
                const k = j - 1
                if (row[j] === ROUND && row[k] === EMPTY) {
                    row[k] = row[j]
                    row[j] = EMPTY
                }
            }
        }
    }
}

function tiltSouth(platform) {
    for (let i = platform.length - 2; i >= 0; i--) {
        for (let j = i; j < platform.length - 1; j++) {
            const currentRow = platform[j]
            const southernRow = platform[j + 1]
            let moved = false
            for (let k = 0; k < currentRow.length; k++) {
                if (currentRow[k] === ROUND && southernRow[k] === EMPTY) {
                    southernRow[k] = currentRow[k]
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

function tiltEast(platform) {
    for (const row of platform) {
        for (let i = row.length - 2; i >= 0; i--) {
            for (let j = i; j < row.length - 1; j++) {
                const k = j + 1
                if (row[j] === ROUND && row[k] === EMPTY) {
                    row[k] = row[j]
                    row[j] = EMPTY
                }
            }
        }
    }
}

function cycle(platform) {
    tiltNorth(platform)
    tiltWest(platform)
    tiltSouth(platform)
    tiltEast(platform)
}

function equals(platform1, platform2) {
    for (let i = 0; i < platform1.length; i++) {
        for (let j = 0; j < platform1[i].length; j++) {
            if (platform1[i][j] !== platform2[i][j]) {
                return false
            }
        }
    }
    return true
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

function clone(platform) {
    return platform.map(row => [...row])
}

function parseFile(fileName) {
    return withLines(fileName, (platform, row) => [...platform, parseRow(row)], [])
}

export function part1(fileName = '14') {
    return parseFile(fileName)
        .then(platform => {
            tiltNorth(platform)

            return calcTotalLoad(platform)
        })
}

export function part2(fileName = '14') {
    return parseFile(fileName)
        .then(platform => {
            const numberOfCycles = 1_000_000_000
            const platforms = []
            for (let i = 0; i < numberOfCycles; i++) {
                platforms.push(clone(platform))
                cycle(platform)
                for (let j = 0; j < platforms.length; j++) {
                    if (equals(platforms[j], platform)) {
                        return calcTotalLoad(platforms[j + (numberOfCycles - j) % (i - j + 1)])
                    }
                }
            }

            return calcTotalLoad(platform)
        })
}
