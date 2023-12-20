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
            const [property, operator] = check
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
    }).then(system => system.parts.reduce((sum, part) => {
        let workflowName = 'in'
        while (system.workflows[workflowName]) {
            workflowName = system.workflows[workflowName].find(rule => rule.predicate(part))?.destination
        }
        return workflowName === 'A'
            ? Object.values(part).reduce((sum, rating) => sum + rating, sum)
            : sum
    }, 0))
}

export function part2(fileName = '19') {
    function parseWorkflow(workflows, line) {
        function parseRule(rule) {
            const [check, destination] = rule.split(':')

            if (destination === undefined) {
                return {
                    transform: {
                        pos: part => part,
                        neg: _ => null,
                    },
                    destination: check,
                }
            }
            const [property, operator] = check
            const value = +check.substring(2)
            const template = transform => part => ({
                    ...part,
                    [property]: {
                        ...part[property],
                        ...transform(part),
                    },
                }
            )
            const transform = operator === '<'
                ? {
                    pos: template(part => ({
                        max: Math.min(part[property].max, value - 1),
                    })),
                    neg: template(part => ({
                        min: Math.max(part[property].min, value),
                    })),
                }
                : {
                    pos: template(part => ({
                        min: Math.max(part[property].min, value + 1),
                    })),
                    neg: template(part => ({
                        max: Math.min(part[property].max, value),
                    })),
                }

            return {
                transform,
                destination,
            }
        }

        const nameLength = line.indexOf('{')
        const name = line.substring(0, nameLength)
        const rules = line.substring(nameLength + 1, line.length - 1).split(',').map(parseRule)

        return {
            ...workflows,
            [name]: rules,
        }
    }

    let parse = parseWorkflow
    return withLines(fileName, (workflows, line) => {
        if (line.length === 0) {
            parse = workflow => workflow
            return workflows
        }
        return parse(workflows, line)
    }, {}).then(workflows => {
        const queue = [{
            workflowName: 'in',
            part: Array.from('xmas').reduce((part, rating) => ({
                ...part,
                [rating]: {
                    min: 1,
                    max: 4000,
                },
            }), {}),
        }]
        for (let i = 0; i < queue.length; i++) {
            let {workflowName, part} = queue[i]
            for (const rule of workflows[workflowName] ?? []) {
                queue.push({
                    workflowName: rule.destination,
                    part: rule.transform.pos(part),
                })
                part = rule.transform.neg(part)
            }
        }
        return queue.filter(item => item.workflowName === 'A')
            .reduce((sum, item) => sum + Object.values(item.part)
                .reduce((product, {min, max}) => product * (max - min + 1), 1), 0)
    })
}
