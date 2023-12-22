import {part1, part2} from './aoc22.js'

test('day 22, part 1', async () => {
    const received = await part1('22_test')
    expect(received).toBe(5)
})

test('day 22, part 2', async () => {
    const received = await part2('22_test')
    expect(received).toBe(7)
})
