import {part1, part2} from './aoc16.js'

test('day 16, part 1', async () => {
    const received = await part1('16_test')
    expect(received).toBe(46)
})

test('day 16, part 2', async () => {
    const received = await part2('16_test')
    expect(received).toBe(51)
})
