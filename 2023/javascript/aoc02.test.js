import { part1, part2 } from './aoc02.js';

test('day 02, part 1', async () => {
    const received = await part1('02_test')
    expect(received).toBe(8);
});

test('day 02, part 2', async () => {
    const received = await part2('02_test')
    expect(received).toBe(2286);
});
