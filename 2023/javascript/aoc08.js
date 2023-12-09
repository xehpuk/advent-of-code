import {withLines} from './utils.js'

export function part1(fileName = '08') {
    return withLines(fileName, (network, line) => {
        if (!network.instructions) {
            return {
                instructions: line,
            }
        }
        if (!line) {
            return {
                ...network,
                nodes: [],
            }
        }
        const [start, left, right] = Array.from(line.matchAll(/[A-Z]+/g), ([match]) => match)
        return {
            ...network,
            nodes: [
                ...network.nodes,
                [start, {
                    L: left,
                    R: right,
                }],
            ],
        }
    }, {}).then(network => ({
        ...network,
        nodes: new Map(network.nodes),
    })).then(({instructions, nodes}) => {
        let node = 'AAA'
        for (let i = 0; ; i++) {
            if (node === 'ZZZ') {
                return i
            }
            node = nodes.get(node)[instructions[i % instructions.length]]
        }
    })
}

export function part2(fileName = '08') {
    throw new Error('TODO')
}
