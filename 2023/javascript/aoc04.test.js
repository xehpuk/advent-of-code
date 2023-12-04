import {part1, part2} from './aoc04.js'

test('day 04, part 1', async () => {
    const received = await part1('04_test')
    expect(received).toBe(13)
})

test('day 04, part 2', async () => {
    const received = await part2('04_test')
    expect(received).toBe(30)
})
