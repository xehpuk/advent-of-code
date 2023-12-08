import {part1, part2} from './aoc07.js'

test('day 07, part 1', async () => {
    const received = await part1('07_test')
    expect(received).toBe(6440)
})

test('day 07, part 2', async () => {
    const received = await part2('07_test')
    expect(received).toBe(5905)
})
