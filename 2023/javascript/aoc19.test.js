import {part1, part2} from './aoc19.js'

test('day 19, part 1', async () => {
    const received = await part1('19_test')
    expect(received).toBe(19114)
})

test('day 19, part 2', async () => {
    const received = await part2('19_test')
    expect(received).toBe(167409079868000)
})
