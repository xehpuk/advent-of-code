import {withLines} from './utils.js'

function createHandleLine(parseLine) {
    return (numbers, line) => [...numbers, parseLine(line)]
}

/**
 * {@link https://www.wolframalpha.com/input?i=solve+d%3C%28t-x%29*x+for+x|WolframAlpha}
 */
function calcNumberOfWins(time, distance) {
    const z = Math.sqrt(time ** 2 - 4 * (distance + 1))
    const time0 = Math.ceil((time - z) / 2)
    const time1 = Math.floor((time + z) / 2)
    return time1 - time0 + 1
}

export function part1(fileName = '06') {
    return withLines(fileName, createHandleLine(line => line.match(/\d+/g).map(n => +n)), [])
        .then(([times, distances]) =>
            times.reduce((product, time, i) => product * calcNumberOfWins(time, distances[i]), 1))
}

export function part2(fileName = '06') {
    return withLines(fileName, createHandleLine(line => +line.replace(/\D/g, '')), [])
        .then(([time, distance]) => calcNumberOfWins(time, distance))
}
