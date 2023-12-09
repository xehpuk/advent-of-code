import {part1, part2} from './aoc08.js'

test('day 08, part 1.1', async () => {
    const received = await part1('08_test1')
    expect(received).toBe(2)
})

test('day 08, part 1.2', async () => {
    const received = await part1('08_test2')
    expect(received).toBe(6)
})

test('day 08, part 2', async () => {
    const received = await part2('08_test3')
    expect(received).toBe(6)
})
