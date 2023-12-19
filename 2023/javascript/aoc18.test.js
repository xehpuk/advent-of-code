import {part1, part2} from './aoc18.js'

test('day 18, part 1', async () => {
    const received = await part1('18_test')
    expect(received).toBe(62)
})

test('day 18, part 2', async () => {
    const received = await part2('18_test')
    expect(received).toBe(952408144115)
})
