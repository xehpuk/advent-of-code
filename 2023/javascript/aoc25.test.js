import {part1} from './aoc25.js'

test('day 25, part 1', async () => {
    const received = await part1('25_test')
    expect(received).toBe(54)
})
