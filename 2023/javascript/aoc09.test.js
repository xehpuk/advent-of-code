import {part1, part2} from './aoc09.js'

test('day 09, part 1', async () => {
    const received = await part1('09_test')
    expect(received).toBe(114)
})

test('day 09, part 2', async () => {
    const received = await part2('09_test')
    expect(received).toBe(2)
})
