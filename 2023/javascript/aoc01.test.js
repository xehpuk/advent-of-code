import {part1, part2} from './aoc01.js'

test('day 01, part 1', async () => {
    const received = await part1('01_test1')
    expect(received).toBe(142)
})

test('day 01, part 2', async () => {
    const received = await part2('01_test2')
    expect(received).toBe(281)
})
