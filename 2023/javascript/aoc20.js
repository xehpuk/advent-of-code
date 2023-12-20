import {withLines} from './utils.js'

export function part1(fileName = '20') {
    return withLines(fileName, (modules, line) => {
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
            ...modules,
            [name]: {
                type,
                destinations,
            },
        }
    }, {}).then(modules => {
        for (const [name, {destinations}] of Object.entries(modules)) {
            for (const destination of destinations) {
                const destinationModule = modules[destination] ?? {}
                switch (destinationModule.type) {
                    case 'flip-flop':
                        destinationModule.value = 0
                        break
                    case 'conjunction':
                        (destinationModule.inputs ??= {})[name] = 0
                        break
                }
            }
        }

        return modules
    }).then(modules => {
        const pulseCount = [0, 0]
        for (let i = 0; i < 1000; i++) {
            const pulses = [{
                source: 'button',
                value: 0,
                destination: 'broadcaster',
            }]
            while (pulses.length > 0) {
                const {source, value, destination: name} = pulses.shift()
                pulseCount[value]++
                const module = modules[name] ?? {}
                switch (module.type) {
                    case 'flip-flop':
                        if (value === 0) {
                            module.value ^= 1
                            for (const destination of module.destinations) {
                                pulses.push({
                                    source: name,
                                    value: module.value,
                                    destination,
                                })
                            }
                        }
                        break
                    case 'conjunction':
                        module.inputs[source] = value
                        const pulseValue = +Object.values(module.inputs).some(input => input === 0)
                        for (const destination of module.destinations) {
                            pulses.push({
                                source: name,
                                value: pulseValue,
                                destination,
                            })
                        }
                        break
                    case 'broadcast':
                        for (const destination of module.destinations) {
                            pulses.push({
                                source: name,
                                value,
                                destination,
                            })
                        }
                        break
                }
            }
        }

        return pulseCount[0] * pulseCount[1]
    })
}

export function part2(fileName = '20') {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
}
