import {withLines} from './utils.js'

export function part1(fileName = '20') {
    return withLines(fileName, (configuration, line) => {
        function parseModule(module) {
            const [type] = module
            switch (type) {
                case '%':
                    return {
                        name: module.substring(1),
                        type: 'flip-flop',
                    }
                case '&':
                    return {
                        name: module.substring(1),
                        type: 'conjunction',
                    }
                case 'b':
                    return {
                        name: module,
                        type: 'broadcast',
                    }
            }
        }

        const [moduleString, destinationsCsv] = line.split(' -> ')
        const {name, type} = parseModule(moduleString)
        const destinations = destinationsCsv.split(', ')

        return {
            ...configuration,
            modules: {
                ...configuration.modules,
                [name]: {
                    type,
                    destinations,
                },
            },
            conjunctionModules: type === 'conjunction'
                ? [...configuration.conjunctionModules, name]
                : configuration.conjunctionModules,
        }
    }, {
        modules: {},
        conjunctionModules: [],
    }).then(({modules, conjunctionModules}) => {
        for (const conjunctionModuleName of conjunctionModules) {
            const conjunctionModule = modules[conjunctionModuleName]
            const inputs = conjunctionModule.inputs ??= {}
            for (const [name, {destinations}] of Object.entries(modules)) {
                if (destinations.includes(conjunctionModuleName)) {
                    inputs[name] = 0
                }
            }
        }

        return modules
    })
}

export function part2(fileName = '20') {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
}
