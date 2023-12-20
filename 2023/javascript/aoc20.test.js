import {part1, part2} from './aoc20.js'

test('day 20, part 1.1', async () => {
    const received = await part1('20_test1')
    expect(received).toBe(32000000)
})

test('day 20, part 1.2', async () => {
    const received = await part1('20_test2')
    expect(received).toBe(11687500)
})
