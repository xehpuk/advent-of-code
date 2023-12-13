import {part1, part2} from './aoc13.js'

test('day 13, part 1.1', async () => {
    const received = await part1('13_test1')
    expect(received).toBe(5)
})

test('day 13, part 1.2', async () => {
    const received = await part1('13_test2')
    expect(received).toBe(400)
})

test('day 13, part 1.3', async () => {
    const received = await part1('13_test3')
    expect(received).toBe(405)
})

test('day 13, part 2.1', async () => {
    const received = await part2('13_test1')
    expect(received).toBe(300)
})

test('day 13, part 2.2', async () => {
    const received = await part2('13_test2')
    expect(received).toBe(100)
})

test('day 13, part 2.3', async () => {
    const received = await part2('13_test3')
    expect(received).toBe(400)
})
