import {part1, part2} from './aoc10.js'

test('day 10, part 1.1', async () => {
    const received = await part1('10_test1')
    expect(received).toBe(4)
})

test('day 10, part 1.2', async () => {
    const received = await part1('10_test2')
    expect(received).toBe(4)
})

test('day 10, part 1.3', async () => {
    const received = await part1('10_test3')
    expect(received).toBe(8)
})

test('day 10, part 1.4', async () => {
    const received = await part1('10_test4')
    expect(received).toBe(8)
})

test('day 10, part 2.1', async () => {
    const received = await part2('10_test5')
    expect(received).toBe(4)
})

test('day 10, part 2.2', async () => {
    const received = await part2('10_test6')
    expect(received).toBe(4)
})

test('day 10, part 2.3', async () => {
    const received = await part2('10_test7')
    expect(received).toBe(4)
})

test('day 10, part 2.4', async () => {
    const received = await part2('10_test8')
    expect(received).toBe(4)
})

test('day 10, part 2.5', async () => {
    const received = await part2('10_test9')
    expect(received).toBe(8)
})

test('day 10, part 2.6', async () => {
    const received = await part2('10_test10')
    expect(received).toBe(10)
})
