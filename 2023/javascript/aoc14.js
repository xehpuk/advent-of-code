import {withLines} from './utils.js'

const regex = /[O#]/g

function parseRow(line) {
    const rocks = {
        'O': new Set(),
        '#': new Set(),
    }
    for (const rock of line.matchAll(regex)) {
        rocks[rock[0]].add(rock.index)
    }
    return rocks
}

export function part1(fileName = '14') {
    return withLines(fileName, (platform, row) => [...platform, parseRow(row)], [])
        .then(platform => {
            for (let i = 1; i < platform.length; i++) {
                for (let j = i; j > 0; j--) {
                    const currentPlatform = platform[j]
                    const upperPlatform = platform[j - 1]
                    for (const roundedRock of currentPlatform['O']) {
                        if (!upperPlatform['O'].has(roundedRock) && !upperPlatform['#'].has(roundedRock)) {
                            currentPlatform['O'].delete(roundedRock)
                            upperPlatform['O'].add(roundedRock)
                        }
                    }
                }
            }
            let sum = 0
            for (let i = 0; i < platform.length; i++) {
                sum += platform[i]['O'].size * (platform.length - i)
            }
            return sum
        })
}

export function part2(fileName = '14') {
    return withLines(fileName, (lines, line) => [...lines, line], [])
}
