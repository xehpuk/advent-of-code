import {part1, part2} from './aoc24.js'

test('day 24, part 1', async () => {
    const received = await part1('24_test', 7, 27)
    expect(received).toBe(2)
})

test('day 24, part 2', async () => {
    const received = await part2('24_test')
    expect(received).toBe(47)
})
