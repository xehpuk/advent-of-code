import {withLines} from './utils.js'

export function part1(fileName = '19') {
    function parseWorkflow(system, line) {
        function parseRule(rule) {
            const [check, destination] = rule.split(':')

            if (destination === undefined) {
                return {
                    predicate: () => true,
                    destination: check,
                }
            }

            const property = check[0]
            const operator = check[1]
            const value = +check.substring(2)
            const predicate = operator === '<'
                ? part => part[property] < value
                : part => part[property] > value

            return {
                predicate,
                destination,
            }
        }

        const nameLength = line.indexOf('{')
        const name = line.substring(0, nameLength)
        const rules = line.substring(nameLength + 1, line.length - 1).split(',').map(parseRule)

        return {
            ...system,
            workflows: {
                ...system.workflows,
                [name]: rules,
            },
        }
    }

    function parsePart(system, line) {
        function parseAssignment(assignment) {
            const [property, value] = assignment.split('=')

            return {
                [property]: +value,
            }
        }

        const part = line.substring(1, line.length - 1)
            .split(',')
            .map(parseAssignment)
            .reduce((part, assignment) => ({
                ...part,
                ...assignment,
            }), {})

        return {
            ...system,
            parts: [
                ...system.parts,
                part,
            ],
        }
    }

    let parse = parseWorkflow
    return withLines(fileName, (system, line) => {
        if (line.length === 0) {
            parse = parsePart
            return system
        }
        return parse(system, line)
    }, {
        workflows: {},
        parts: [],
    }).then(system =>
        system.parts.map(part => {
            let workflowName = 'in'
            while (system.workflows[workflowName]) {
                for (const rule of system.workflows[workflowName]) {
                    if (rule.predicate(part)) {
                        workflowName = rule.destination
                        break
                    }
                }
            }
            return workflowName === 'A'
                ? Object.values(part).reduce((rating, value) => rating + value, 0)
                : 0
        }).reduce((sum, rating) => sum + rating, 0))
}

export function part2(fileName = '19') {
    return withLines(fileName, (lines, line, index) => [...lines, line], [])
}
