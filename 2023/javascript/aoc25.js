import {withLines} from './utils.js'

function toGraphviz(connections) {
    return `strict graph {
    ${Object.entries(connections)
        .map(([k, v]) => `${k} -- {${v.join(' ')}}`)
        .join('\n')}
    }`
}

function countComponents(connections) {
    const components = new Set([Object.keys(connections)[0]])
    for (const component of components) {
        for (const connection of connections[component]) {
            components.add(connection)
        }
    }
    return components.size
}

export function part1(fileName = '25') {
    return withLines(fileName, ({inflated, deflated}, line) => {
        const [first, ...rest] = line.match(/[a-z]{3}/g)
        deflated[first] ??= new Set()
        for (const component of rest) {
            deflated[first].add(component)
            deflated[component] ??= new Set()
            deflated[component].add(first)
        }
        return {
            inflated: [...inflated, ...rest.map(component => [first, component])],
            deflated,
        }
    }, {
        inflated: [],
        deflated: {},
    }).then(({inflated, deflated}) => {
        // apparently too slow for real input
        const totalCount = countComponents(deflated)
        for (let i = 0; i < inflated.length - 2; i++) {
            for (let j = i + 1; j < inflated.length - 1; j++) {
                for (let k = j + 1; k < inflated.length; k++) {
                    const originals = {}
                    for (const x of [i, j, k]) {
                        const [left, right] = inflated[x]
                        if (!(left in originals)) {
                            originals[left] = new Set(deflated[left])
                        }
                        if (!(right in originals)) {
                            originals[right] = new Set(deflated[right])
                        }
                        deflated[left].delete(right)
                        deflated[right].delete(left)
                    }
                    const count = countComponents(deflated)
                    if (count !== totalCount) {
                        return count * (totalCount - count)
                    }
                    for (const [first, rest] of Object.entries(originals)) {
                        deflated[first] = rest
                    }
                }
            }
        }
    })
}
