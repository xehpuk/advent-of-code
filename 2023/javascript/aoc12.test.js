import {part1, part2} from './aoc12.js'

test('day 12, part 1.1', async () => {
    const received = await part1('12_test1')
    expect(received).toBe(21)
})

test('day 12, part 1.2', async () => {
    const received = await part1('12_test2')
    expect(received).toBe(1)
})

test('day 12, part 1.3', async () => {
    const received = await part1('12_test3')
    expect(received).toBe(4)
})

test('day 12, part 1.4', async () => {
    const received = await part1('12_test4')
    expect(received).toBe(1)
})

test('day 12, part 1.5', async () => {
    const received = await part1('12_test5')
    expect(received).toBe(1)
})

test('day 12, part 1.6', async () => {
    const received = await part1('12_test6')
    expect(received).toBe(4)
})

test('day 12, part 1.7', async () => {
    const received = await part1('12_test7')
    expect(received).toBe(10)
})

test('day 12, part 2', async () => {
    const received = await part2('12_test1')
    expect(received).toBe(525152)
})
