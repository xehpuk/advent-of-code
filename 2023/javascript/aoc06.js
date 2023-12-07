import {withLines} from './utils.js'

function parseLine(line) {
    return line.match(/\d+/g).map(n => +n)
}

export function part1(fileName = '06') {
    return withLines(fileName, (numbers, line) => [...numbers, parseLine(line)], [])
        .then(([times, distances]) =>
            times.reduce((product, time, i) => {
                const distance = distances[i]
                const z = Math.sqrt(time ** 2 - 4 * distance)
                const time0 = Math.floor((time - z) / 2 + 1)
                const time1 = Math.ceil((time + z) / 2 - 1)
                return product * (time1 - time0 + 1)
            }, 1))
}

export function part2(fileName = '06') {
    throw new Error('TODO')
}
