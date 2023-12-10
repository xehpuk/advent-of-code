import * as day01 from './aoc01.js'
import * as day02 from './aoc02.js'
import * as day03 from './aoc03.js'
import * as day04 from './aoc04.js'
import * as day06 from './aoc06.js'
import * as day07 from './aoc07.js'
import * as day08 from './aoc08.js'
import * as day09 from './aoc09.js'
import * as day10 from './aoc10.js'

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
