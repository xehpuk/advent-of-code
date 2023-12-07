import {part1, part2} from './aoc06.js'

test('day 06, part 1', async () => {
    const received = await part1('06_test')
    expect(received).toBe(288)
})

test('day 06, part 2', async () => {
    const received = await part2('06_test')
    expect(received).toBe(71503)
})
