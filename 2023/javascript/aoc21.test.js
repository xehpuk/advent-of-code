import {part1, part2} from './aoc21.js'

test('day 21, part 1', async () => {
    const received = await part1('21_test', 6)
    expect(received).toBe(16)
})

test('day 21, part 2.1', async () => {
    const received = await part2('21_test', 6)
    expect(received).toBe(16)
})

test('day 21, part 2.2', async () => {
    const received = await part2('21_test', 10)
    expect(received).toBe(50)
})

test('day 21, part 2.3', async () => {
    const received = await part2('21_test', 50)
    expect(received).toBe(1594)
})

test('day 21, part 2.4', async () => {
    const received = await part2('21_test', 100)
    expect(received).toBe(6536)
})

test('day 21, part 2.5', async () => {
    const received = await part2('21_test', 500)
    expect(received).toBe(167004)
})

test('day 21, part 2.6', async () => {
    const received = await part2('21_test', 1000)
    expect(received).toBe(668697)
})

test('day 21, part 2.7', async () => {
    const received = await part2('21_test', 5000)
    expect(received).toBe(16733044)
})
