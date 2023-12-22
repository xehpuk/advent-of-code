import * as day01 from './aoc01.js'
import * as day02 from './aoc02.js'
import * as day03 from './aoc03.js'
import * as day04 from './aoc04.js'
import * as day06 from './aoc06.js'
import * as day07 from './aoc07.js'
import * as day08 from './aoc08.js'
import * as day09 from './aoc09.js'
import * as day10 from './aoc10.js'
import * as day11 from './aoc11.js'
import * as day12 from './aoc12.js'
import * as day13 from './aoc13.js'
import * as day14 from './aoc14.js'
import * as day15 from './aoc15.js'
import * as day16 from './aoc16.js'
import * as day18 from './aoc18.js'
import * as day19 from './aoc19.js'
import * as day20 from './aoc20.js'
import * as day21 from './aoc21.js'
import * as day22 from './aoc22.js'

async function log(message, part) {
    console.time(message)
    console.log(`${message}: ${await part()}`)
    console.timeEnd(message)
}

await log('day 01, part 1', day01.part1)
await log('day 01, part 2', day01.part2)
await log('day 02, part 1', day02.part1)
await log('day 02, part 2', day02.part2)
await log('day 03, part 1', day03.part1)
await log('day 03, part 2', day03.part2)
await log('day 04, part 1', day04.part1)
await log('day 04, part 2', day04.part2)
await log('day 06, part 1', day06.part1)
await log('day 06, part 2', day06.part2)
await log('day 07, part 1', day07.part1)
await log('day 07, part 2', day07.part2)
await log('day 08, part 1', day08.part1)
await log('day 08, part 2', day08.part2)
await log('day 09, part 1', day09.part1)
await log('day 09, part 2', day09.part2)
await log('day 10, part 1', day10.part1)
await log('day 10, part 2', day10.part2)
await log('day 11, part 1', day11.part1)
await log('day 11, part 2', day11.part2)
await log('day 12, part 1', day12.part1)
await log('day 13, part 1', day13.part1)
await log('day 13, part 2', day13.part2)
await log('day 14, part 1', day14.part1)
await log('day 14, part 2', day14.part2)
await log('day 15, part 1', day15.part1)
await log('day 15, part 2', day15.part2)
await log('day 16, part 1', day16.part1)
await log('day 16, part 2', day16.part2)
await log('day 18, part 1', day18.part1)
await log('day 19, part 1', day19.part1)
await log('day 19, part 2', day19.part2)
await log('day 20, part 1', day20.part1)
await log('day 20, part 2', day20.part2)
await log('day 21, part 1', day21.part1)
await log('day 22, part 1', day22.part1)
await log('day 22, part 2', day22.part2)
