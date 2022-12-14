use super::Day;

use std::collections::{HashMap, HashSet};

pub struct Day12;

type Position = (usize, usize);

#[derive(Debug)]
struct Heightmap {
    grid: HashMap<Position, Node>,
    start: Position,
    end: Position,
}

impl Heightmap {
    fn get_neighbours(&self, position: Position) -> Vec<Position> {
        let mut neighbours = Vec::new();

        if let Some(Node { height, .. }) = self.grid.get(&position) {
            if let Some(x) = position.0.checked_sub(1) {
                if self.is_walkable((x, position.1), *height) {
                    neighbours.push((x, position.1));
                }
            }
            if self.is_walkable((position.0 + 1, position.1), *height) {
                neighbours.push((position.0 + 1, position.1));
            }
            if let Some(y) = position.1.checked_sub(1) {
                if self.is_walkable((position.0, y), *height) {
                    neighbours.push((position.0, y));
                }
            }
            if self.is_walkable((position.0, position.1 + 1), *height) {
                neighbours.push((position.0, position.1 + 1));
            }
        }

        neighbours
    }

    fn is_walkable(&self, position: Position, height: u32) -> bool {
        if let Some(node) = self.grid.get(&position) {
            return height + 1 >= node.height;
        }
        false
    }

    fn retrace_path(&self) -> Option<i32> {
        let mut path = Vec::new();
        let mut current = self.end;

        while current != self.start {
            path.push(current);
            current = self.grid.get(&current)?.parent?;
        }

        path.reverse();

        println!("{path:?}");

        Some(path.len() as i32)
    }
}

fn parse_heightmap<'a, I: Iterator<Item = &'a str>>(input: I) -> Option<Heightmap> {
    let mut grid = HashMap::new();
    let mut start = None;
    let mut end = None;

    for (y, heights) in input.map(str::chars).enumerate() {
        for (x, height) in heights.enumerate() {
            let position = (x, y);
            grid.insert(
                position,
                Node::new(
                    position,
                    match height {
                        'S' => {
                            start = Some(position);
                            'a'
                        }
                        'E' => {
                            end = Some(position);
                            'z'
                        }
                        _ => height,
                    },
                ),
            );
        }
    }

    Some(Heightmap {
        grid,
        start: start?,
        end: end?,
    })
}

#[derive(Debug)]
struct Node {
    position: Position,
    height: u32,
    g_cost: usize,
    h_cost: usize,
    parent: Option<Position>,
}

impl Node {
    fn new(position: (usize, usize), height: char) -> Self {
        Self {
            position,
            height: u32::from(height) - u32::from('a'),
            g_cost: 0,
            h_cost: 0,
            parent: None,
        }
    }

    fn f_cost(&self) -> usize {
        self.g_cost + self.h_cost
    }
}

fn get_distance(start: Position, end: Position) -> usize {
    start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
}

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, i32> for Day12 {
    /// Ported (very unoptimized) C# version from:
    /// https://youtu.be/mZfyt03LDH4
    /// Sebastian Lague
    /// A* Pathfinding (E03: algorithm implementation)
    fn part1(input: I) -> Option<i32> {
        let mut heightmap = parse_heightmap(input)?;

        let mut open_set = Vec::new();
        let mut closed_set = HashSet::new();
        open_set.push(heightmap.start);

        while !open_set.is_empty() {
            let current = open_set[0];
            let mut current_node = heightmap.grid.get(&current)?;
            for i in 1..open_set.len() {
                let next = open_set[i];
                let next_node = heightmap.grid.get(&next)?;
                if next_node.f_cost() < current_node.f_cost()
                    || next_node.f_cost() == current_node.f_cost()
                        && next_node.h_cost < current_node.h_cost
                {
                    current_node = next_node;
                }
            }
            let current = current_node.position;
            open_set.retain(|&node| node != current);
            closed_set.insert(current);

            if current == heightmap.end {
                return heightmap.retrace_path();
            }

            let current_g_cost = current_node.g_cost;
            for neighbour in heightmap.get_neighbours(current) {
                if closed_set.contains(&neighbour) {
                    continue;
                }

                let new_movement_cost_to_neighbour =
                    current_g_cost + get_distance(current, neighbour);
                let neighbour_node = heightmap.grid.get_mut(&neighbour)?;

                if new_movement_cost_to_neighbour < neighbour_node.g_cost
                    || !open_set.contains(&neighbour)
                {
                    neighbour_node.g_cost = new_movement_cost_to_neighbour;
                    neighbour_node.h_cost = get_distance(neighbour, heightmap.end);
                    neighbour_node.parent = Some(current);
                    if !open_set.contains(&neighbour) {
                        open_set.push(neighbour);
                    }
                }
            }
        }

        None
    }

    fn part2(_input: I) -> Option<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day12};
    use std::str::Lines;

    const INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<i32> {
        Day12::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        Day12::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(31), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
