import {withLines} from './utils.js'

export function part1(fileName = '__') {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
}

export function part2(fileName = '__') {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
}
