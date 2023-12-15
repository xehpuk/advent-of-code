import {withLines} from './utils.js'

function hash(step) {
    let hash = 0
    for (let i = 0; i < step.length; i++) {
        hash = (hash + step.charCodeAt(i)) * 17 % 256
    }
    return hash
}

export function part1(fileName = '15') {
    return withLines(fileName, (sum, line) => line.split(',')
        .map(hash)
        .reduce((sum, hash) => sum + hash, sum), 0)
}

export function part2(fileName = '15') {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
}
