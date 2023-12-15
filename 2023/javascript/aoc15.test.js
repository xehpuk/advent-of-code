import {part1, part2} from './aoc15.js'

test('day 15, part 1.1', async () => {
    const received = await part1('15_test1')
    expect(received).toBe(52)
})

test('day 15, part 1.2', async () => {
    const received = await part1('15_test2')
    expect(received).toBe(1320)
})

test('day 15, part 2', async () => {
    const received = await part2('15_test2')
    expect(received).toBe(145)
})
