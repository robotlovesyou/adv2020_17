use indoc::indoc;
use std::collections::HashSet;

fn get_3d_neigbour_coords(cx: i64, cy: i64, cz: i64) -> Vec<(i64, i64, i64)> {
    let mut neighbours = Vec::new();
    for x in cx - 1..=cx + 1 {
        for y in cy - 1..=cy + 1 {
            for z in cz - 1..=cz + 1 {
                if x != cx || y != cy || z != cz {
                    neighbours.push((x, y, z))
                }
            }
        }
    }

    neighbours
}

fn get_4d_neigbour_coords(cw: i64, cx: i64, cy: i64, cz: i64) -> Vec<(i64, i64, i64, i64)> {
    let mut neighbours = Vec::new();
    for w in cw - 1..=cw + 1 {
        for x in cx - 1..=cx + 1 {
            for y in cy - 1..=cy + 1 {
                for z in cz - 1..=cz + 1 {
                    if w != cw || x != cx || y != cy || z != cz {
                        neighbours.push((w, x, y, z))
                    }
                }
            }
        }
    }

    neighbours
}

const PUZZLE_INPUT: &str = indoc! {"
    ...#..#.
    ..##.##.
    ..#.....
    ....#...
    #.##...#
    ####..##
    ...##.#.
    #.#.#...
"};

struct ThreeSpace {
    grid: HashSet<(i64, i64, i64)>,
}

impl ThreeSpace {
    fn new<'a>(initial_state: impl Iterator<Item = &'a str>) -> ThreeSpace {
        let z = 0i64;
        let mut grid = HashSet::new();
        for (y, line) in initial_state.enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    grid.insert((x as i64, y as i64, z));
                }
            }
        }

        ThreeSpace { grid }
    }

    pub fn cycle(&mut self) {
        let mut next = HashSet::new();

        // get the list of coords to examine (all active coords and all their neighbours)
        // for each coord to examine, get its neighbour count and mark it as active or inactive in next
        // replace self.grid with next
        for (x, y, z) in self.get_coords_to_examine() {
            let active_count = self.get_active_neighbour_count(x, y, z);
            if (self.grid.contains(&(x, y, z)) && active_count == 2) || active_count == 3 {
                next.insert((x, y, z));
            }
        }
        self.grid = next;
    }

    pub fn cycle_n_times(&mut self, n: usize) {
        for _ in 0..n {
            self.cycle();
        }
    }

    fn get_coords_to_examine(&self) -> Vec<(i64, i64, i64)> {
        let mut to_examine = HashSet::new();
        for (x, y, z) in self.grid.iter() {
            for coord in get_3d_neigbour_coords(*x, *y, *z) {
                to_examine.insert(coord);
            }
        }

        to_examine.into_iter().collect()
    }

    fn get_active_neighbour_count(&self, x: i64, y: i64, z: i64) -> usize {
        let mut count = 0;
        for coord in get_3d_neigbour_coords(x, y, z) {
            if self.grid.contains(&coord) {
                count += 1;
            }
        }
        count
    }

    pub fn get_active_count(&self) -> usize {
        self.grid.len()
    }
}

struct FourSpace {
    grid: HashSet<(i64, i64, i64, i64)>,
}

impl FourSpace {
    fn new<'a>(initial_state: impl Iterator<Item = &'a str>) -> FourSpace {
        let (w, z) = (0i64, 0i64);
        let mut grid = HashSet::new();
        for (y, line) in initial_state.enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    grid.insert((w, x as i64, y as i64, z));
                }
            }
        }

        FourSpace { grid }
    }

    pub fn cycle(&mut self) {
        let mut next = HashSet::new();

        // get the list of coords to examine (all active coords and all their neighbours)
        // for each coord to examine, get its neighbour count and mark it as active or inactive in next
        // replace self.grid with next
        for (w, x, y, z) in self.get_coords_to_examine() {
            let active_count = self.get_active_neighbour_count(w, x, y, z);
            if (self.grid.contains(&(w, x, y, z)) && active_count == 2) || active_count == 3 {
                next.insert((w, x, y, z));
            }
        }
        self.grid = next;
    }

    pub fn cycle_n_times(&mut self, n: usize) {
        for _ in 0..n {
            self.cycle();
        }
    }

    fn get_coords_to_examine(&self) -> Vec<(i64, i64, i64, i64)> {
        let mut to_examine = HashSet::new();
        for (w, x, y, z) in self.grid.iter() {
            for coord in get_4d_neigbour_coords(*w, *x, *y, *z) {
                to_examine.insert(coord);
            }
        }

        to_examine.into_iter().collect()
    }

    fn get_active_neighbour_count(&self, w: i64, x: i64, y: i64, z: i64) -> usize {
        let mut count = 0;
        for coord in get_4d_neigbour_coords(w, x, y, z) {
            if self.grid.contains(&coord) {
                count += 1;
            }
        }
        count
    }

    pub fn get_active_count(&self) -> usize {
        self.grid.len()
    }
}

fn main() {
    let mut space = ThreeSpace::new(PUZZLE_INPUT.lines());
    space.cycle_n_times(6);
    println!("part 1 answer: {}", space.get_active_count());

    let mut four_space = FourSpace::new(PUZZLE_INPUT.lines());
    four_space.cycle_n_times(6);
    println!("part 2 answer: {}", four_space.get_active_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        .#.
        ..#
        ###
    "};

    #[test]
    fn it_successfully_initializes() {
        let space = ThreeSpace::new(TEST_INPUT.lines());

        assert!(space.grid.contains(&(1, 0, 0)));
        assert!(!space.grid.contains(&(0, 0, 0)));
        assert!(space.grid.contains(&(2, 1, 0)));
    }

    #[test]
    fn it_successfully_cycles() {
        let mut space = ThreeSpace::new(TEST_INPUT.lines());
        space.cycle();
        assert_eq!(11, space.get_active_count());
    }

    #[test]
    fn it_successfully_cycles_6_times() {
        let mut space = ThreeSpace::new(TEST_INPUT.lines());
        space.cycle_n_times(6);
        assert_eq!(112, space.get_active_count());
    }

    #[test]
    fn it_successfully_cycles_4_space() {
        let mut space = FourSpace::new(TEST_INPUT.lines());
        space.cycle();
        assert_eq!(29, space.get_active_count());
    }

    #[test]
    fn it_successfully_cycles_4_space_6_times() {
        let mut space = FourSpace::new(TEST_INPUT.lines());
        space.cycle_n_times(6);
        assert_eq!(848, space.get_active_count());
    }
}
