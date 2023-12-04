import { part1, part2 } from './aoc03.js';

test('day 03, part 1', async () => {
    const received = await part1('03_test')
    expect(received).toBe(4361);
});

test('day 03, part 2', async () => {
    const received = await part2('03_test')
    expect(received).toBe(467835);
});
