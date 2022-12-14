import { open } from 'node:fs/promises'
import { join } from 'node:path'

const file = await open(join('..', '13.txt'))
const lines = file.readLines()[Symbol.asyncIterator]()

for (let i = 1, sum = 0;; i++) {
    const first = await lines.next()
    const second = await lines.next()

    lines.next() // skip empty

    if (first.done || second.done) {
        console.log(sum)
        break
    }

    if (compare(JSON.parse(first.value), JSON.parse(second.value)) <= 0) {
        sum += i
    }
}

function compare(left, right) {
    const leftNumber = typeof left === 'number'
    const rightNumber = typeof right === 'number'

    if (leftNumber && rightNumber) {
        return left - right
    }

    const leftList = leftNumber ? [left] : left
    const rightList = rightNumber ? [right] : right

    const minLength = Math.min(leftList.length, rightList.length)

    for (let i = 0; i < minLength; i++) {
        const order = compare(leftList[i], rightList[i])

        if (order !== 0) {
            return order
        }
    }

    return leftList.length - rightList.length
}
