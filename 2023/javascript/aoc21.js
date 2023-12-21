import {withLines} from './utils.js'

// only works on real input, not example
export function part1(fileName = '21', steps = 64) {
    return withLines(fileName, (lines, line) => [...lines, line], [])
        .then(garden => {
            const xStart = garden[0].length >> 1
            const yStart = garden.length >> 1

            let count = 0
            for (let i = 0; i < 2 * steps + 1; i++) {
                const y = i + yStart - steps
                for (let x = xStart - steps + Math.abs(y - yStart); x <= xStart + steps - Math.abs(y - yStart); x += 2) {
                    if (y in garden && garden[y][x] === '.') {
                        count++
                    }
                }
            }
            return count
        })
}

export function part2(fileName = '21', steps = 26501365) {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
}
