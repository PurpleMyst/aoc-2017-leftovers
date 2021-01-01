use std::{collections::HashMap, fmt::Display};

// TODO: synthetize the 2x2 and 3x3 tiles into a u16
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Tile {
    side: usize,
    cells: Vec<bool>,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.cells.chunks_exact(self.side).enumerate() {
            if i != 0 {
                if f.alternate() {
                    writeln!(f)?;
                } else {
                    write!(f, "/")?;
                }
            }

            for &col in row {
                if col {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}

impl Tile {
    fn from_input(input: &str) -> Self {
        let rows = input.split('/');
        let mut side = 0;
        let mut tiles = Vec::with_capacity(3 * 3);

        for row in rows {
            side += 1;

            for ch in row.bytes() {
                tiles.push(ch == b'#');
            }
        }

        Self { side, cells: tiles }
    }

    fn flip_horizontally(mut self) -> Self {
        self.cells
            .chunks_exact_mut(self.side)
            .for_each(|row| row.reverse());

        self
    }

    fn rotate_cw(mut self) -> Self {
        for i in 0..self.side / 2 {
            for j in i..self.side - (i + 1) {
                let temp = self.cells[i * self.side + j];

                self.cells
                    .swap(i * self.side + j, (self.side - 1 - j) * self.side + i);
                self.cells.swap(
                    (self.side - 1 - j) * self.side + i,
                    (self.side - 1 - i) * self.side + (self.side - 1 - j),
                );
                self.cells.swap(
                    (self.side - 1 - i) * self.side + (self.side - 1 - j),
                    j * self.side + (self.side - 1 - i),
                );

                self.cells[j * self.side + self.side - 1 - i] = temp;
            }
        }

        self
    }

    pub fn possible_transformations(self) -> [Self; 8] {
        [
            self.clone(),
            self.clone().rotate_cw(),
            self.clone().rotate_cw().rotate_cw(),
            self.clone().rotate_cw().rotate_cw().rotate_cw(),
            self.clone().flip_horizontally(),
            self.clone().flip_horizontally().rotate_cw(),
            self.clone().flip_horizontally().rotate_cw().rotate_cw(),
            self.flip_horizontally().rotate_cw().rotate_cw().rotate_cw(),
        ]
    }

    pub fn two_by_two(top: &[bool], btm: &[bool]) -> Self {
        let mut tile = Tile {
            side: 2,
            cells: vec![],
        };
        tile.cells.extend_from_slice(&top);
        tile.cells.extend_from_slice(&btm);
        tile
    }

    pub fn three_by_three(top: &[bool], mid: &[bool], btm: &[bool]) -> Self {
        let mut tile = Tile {
            side: 3,
            cells: vec![],
        };
        tile.cells.extend_from_slice(&top);
        tile.cells.extend_from_slice(&mid);
        tile.cells.extend_from_slice(&btm);
        tile
    }
}

fn enhance(
    cells: &mut Vec<bool>,
    side: &mut usize,
    next_cells: &mut Vec<bool>,
    new: &mut Vec<Vec<bool>>,
    rules: &HashMap<Tile, Tile>,
) {
    next_cells.clear();

    if *side % 2 == 0 {
        let mut rows = cells.chunks_exact(*side);

        *side = *side / 2 * 3;

        while let Some((top, btm)) = rows.next().zip(rows.next()) {
            new.iter_mut().for_each(|r| r.clear());

            for (top, btm) in top.chunks_exact(2).zip(btm.chunks_exact(2)) {
                let tile = Tile::two_by_two(top, btm);

                let replacement = rules.get(&tile).unwrap();
                assert_eq!(replacement.side, 3);

                for (row, dest) in replacement
                    .cells
                    .chunks_exact(replacement.side)
                    .zip(new.iter_mut())
                {
                    dest.extend_from_slice(row);
                }
            }

            for a in new.iter().take(3) {
                next_cells.extend_from_slice(a)
            }
        }
    } else {
        let mut rows = cells.chunks_exact(*side);

        *side = *side / 3 * 4;

        while let Some(((top, mid), btm)) = rows.next().zip(rows.next()).zip(rows.next()) {
            new.iter_mut().for_each(|r| r.clear());

            for ((top, mid), btm) in top
                .chunks_exact(3)
                .zip(mid.chunks_exact(3))
                .zip(btm.chunks_exact(3))
            {
                let tile = Tile::three_by_three(top, mid, btm);

                let replacement = rules.get(&tile).unwrap();
                assert_eq!(replacement.side, 4);

                for (row, dest) in replacement
                    .cells
                    .chunks_exact(replacement.side)
                    .zip(new.iter_mut())
                {
                    dest.extend_from_slice(row);
                }
            }
            *side = new[0].len();

            for a in new.iter() {
                next_cells.extend_from_slice(a);
            }
        }
    }

    cells.clear();
    cells.extend_from_slice(&next_cells)
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut rules = HashMap::new();

    for rule in include_str!("input.txt").lines() {
        let mut sides = rule.splitn(2, " => ");
        let lhs = Tile::from_input(sides.next().unwrap());
        let rhs = Tile::from_input(sides.next().unwrap());

        rules.extend(
            array_iterator::ArrayIterator::new(lhs.possible_transformations())
                .map(|lhs| (lhs, rhs.clone())),
        );
    }

    let mut cells = vec![false, true, false, false, false, true, true, true, true];
    let mut side = 3;

    let mut next_cells = Vec::new();
    let mut new = vec![Vec::new(); 4];

    for _ in 0..5 {
        enhance(&mut cells, &mut side, &mut next_cells, &mut new, &rules);
    }

    let p1 = cells.iter().filter(|&&x| x).count();

    for _ in 5..18 {
        enhance(&mut cells, &mut side, &mut next_cells, &mut new, &rules);
    }

    let p2 = cells.iter().filter(|&&x| x).count();

    (p1, p2)
}
