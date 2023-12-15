import {withLines} from './utils.js'

function parseLine(line) {
    return line.split(',')
}

function hash(step) {
    let hash = 0
    for (let i = 0; i < step.length; i++) {
        hash = (hash + step.charCodeAt(i)) * 17 % 256
    }
    return hash
}

export function part1(fileName = '15') {
    return withLines(fileName, (sum, line) => parseLine(line)
            .map(hash)
            .reduce((sum, hash) => sum + hash, sum),
        0)
}

export function part2(fileName = '15') {
    function parseStep(step) {
        const [label] = step.match(/[a-z]+/)
        const operation = step[label.length]
        const focalLength = +step.substring(label.length + 1)

        return {
            label,
            operation,
            focalLength,
        }
    }

    function executeStep(boxes, step) {
        const box = boxes[hash(step.label)] ??= []
        const labelIndex = box.findIndex(lens => lens.label === step.label)
        switch (step.operation) {
            case '=':
                if (labelIndex !== -1) {
                    box[labelIndex].focalLength = step.focalLength
                } else {
                    box.push(step)
                }
                break
            case '-':
                if (labelIndex !== -1) {
                    box.splice(labelIndex, 1)
                }
                break
        }
        return boxes
    }

    function calcFocusingPower(sum, box, i) {
        return box.reduce((sum, lens, slot) => sum + (1 + i) * (1 + slot) * lens.focalLength, sum)
    }

    return withLines(fileName, (sum, line) => parseLine(line)
            .map(parseStep)
            .reduce(executeStep, [])
            .reduce(calcFocusingPower, 0),
        0)
}
