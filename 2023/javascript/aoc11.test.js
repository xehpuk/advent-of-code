import {part1, part2} from './aoc11.js'

test('day 11, part 1', async () => {
    const received = await part1('11_test')
    expect(received).toBe(374)
})

test('day 11, part 2.1', async () => {
    const received = await part2('11_test', 10)
    expect(received).toBe(1030)
})

test('day 11, part 2.2', async () => {
    const received = await part2('11_test', 100)
    expect(received).toBe(8410)
})
