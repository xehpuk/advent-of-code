import {withLines} from './utils.js'

function parseLine(line) {
    const [game, semicolonSeparatedListOfSubsetsOfCubes] = line.split(': ')
    const [, gameId] = game.split(' ')
    const listOfSubsetsOfCubes = semicolonSeparatedListOfSubsetsOfCubes.split('; ')
    const subsetsOfCubes = listOfSubsetsOfCubes.flatMap(subsetsOfCubes => subsetsOfCubes.split(', '))
    const cubes = subsetsOfCubes.map(subsetOfCubes => subsetOfCubes.split(' '))
        .reduce((cubes, [count, color]) => ({
            ...cubes,
            [color]: Math.max(cubes[color] ?? 0, Number.parseInt(count))
        }), {})

    return {
        id: Number.parseInt(gameId),
        cubes
    }
}

export function part1(fileName = '02') {
    const bag = {
        red: 12,
        green: 13,
        blue: 14,
    }

    return withLines(fileName, (value, line) => {
        const game = parseLine(line)
        if (Object.entries(game.cubes).every(([color, count]) => bag[color] >= count)) {
            return value + game.id
        }
        return value
    }, 0)
}

export function part2(fileName = '02') {
    return withLines(fileName, (value, line) => {
        const game = parseLine(line)
        const power = Object.values(game.cubes).reduce((power, count) => power * count, 1)
        return value + power
    }, 0)
}
