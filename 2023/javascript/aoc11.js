import {withLines} from './utils.js'

function expandAndSumDistances(expansion) {
    return fileName => withLines(fileName, (galaxies, line, index) =>
        [...galaxies, ...Array.from(line.matchAll(/#/g), match => ({
            x: match.index,
            y: index,
        }))], []).then(galaxies => {
        function expand(dimension) {
            let i = galaxies.length - 1
            g: for (let coordinate = galaxies[galaxies.length - 1][dimension] - 1; coordinate >= 0; coordinate--) {
                while (i >= 0) {
                    const galaxyCoordinate = galaxies[i][dimension]
                    if (galaxyCoordinate < coordinate) {
                        break
                    }
                    i--
                    if (galaxyCoordinate === coordinate) {
                        continue g
                    }
                }
                for (let j = i + 1; j < galaxies.length; j++) {
                    galaxies[j][dimension] += expansion - 1
                }
            }
        }

        expand('y')
        galaxies.sort((galaxy1, galaxy2) => galaxy1.x - galaxy2.x)
        expand('x')

        return galaxies.reduce((sum, galaxy, i) => {
            for (let j = i + 1; j < galaxies.length; j++) {
                sum += galaxies[j].x - galaxy.x + Math.abs(galaxies[j].y - galaxy.y)
            }
            return sum
        }, 0)
    })
}

export function part1(fileName = '11') {
    return expandAndSumDistances(2)(fileName)
}

export function part2(fileName = '11', expansion = 1_000_000) {
    return expandAndSumDistances(expansion)(fileName)
}
