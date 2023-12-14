import {part1, part2} from './aoc14.js'

test('day 14, part 1', async () => {
    const received = await part1('14_test')
    expect(received).toBe(136)
})

test('day 14, part 2', async () => {
    const received = await part2('14_test')
    expect(received).toBe(64)
})
