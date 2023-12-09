import lcm from 'lcm'

import {withLines} from './utils.js'

function parseNetwork(fileName, regex) {
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
        const [start, left, right] = Array.from(line.matchAll(regex), ([match]) => match)
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
    }))
}

export function part1(fileName = '08') {
    return parseNetwork(fileName, /[A-Z]+/g)
        .then(({instructions, nodes}) => {
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
    return parseNetwork(fileName, /[A-Z0-9]+/g)
        .then(({instructions, nodes}) =>
            Array.from(nodes.keys())
                .filter(node => node.endsWith('A'))
                .map(node => {
                    for (let i = 0; ; i++) {
                        if (node.endsWith('Z')) {
                            return i
                        }
                        node = nodes.get(node)[instructions[i % instructions.length]]
                    }
                }).reduce(lcm))
    // "correct" solution, but too slow
    // let currentNodes = Array.from(nodes.keys()).filter(node => node.endsWith('A'))
    // for (let i = 0; ; i++) {
    //     if (currentNodes.every(node => node.endsWith('Z'))) {
    //         return i
    //     }
    //     currentNodes = currentNodes.map(node => nodes.get(node)[instructions[i % instructions.length]])
    // }
}
