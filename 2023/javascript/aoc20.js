import {withLines} from './utils.js'

function parseModules(modules, line) {
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
}

function initModules(modules) {
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
}

function process(pulse, module) {
    switch (module.type) {
        case 'flip-flop':
            if (pulse.value === 0) {
                return module.value ^= 1
            }
            return
        case 'conjunction':
            module.inputs[pulse.source] = pulse.value
            return +Object.values(module.inputs).some(input => input === 0)
        case 'broadcast':
            return pulse.value
    }
}

export function part1(fileName = '20') {
    return withLines(fileName, parseModules, {})
        .then(initModules)
        .then(modules => {
            const pulseCount = [0, 0]

            for (let i = 0; i < 1000; i++) {
                const pulses = [{
                    source: 'button',
                    value: 0,
                    destination: 'broadcaster',
                }]
                while (pulses.length > 0) {
                    const pulse = pulses.shift()
                    pulseCount[pulse.value]++
                    const module = modules[pulse.destination] ?? {}
                    const value = process(pulse, module)
                    if (value !== undefined) {
                        for (const destination of module.destinations) {
                            pulses.push({
                                source: pulse.destination,
                                value,
                                destination,
                            })
                        }
                    }
                }
            }

            const [lowPulseCount, highPulseCount] = pulseCount
            return lowPulseCount * highPulseCount
        })
}

export function part2(fileName = '20') {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
}
