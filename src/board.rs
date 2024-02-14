use rand::{self, Rng};
use std::i16;

#[derive(Debug)]
pub struct Board {
    pub items: Vec<Vec<i16>>,
    modified: bool,
}

impl Board {
    fn transposed(&mut self) -> Board {
        return Board {
            items: vec![
                vec![
                    self.items[0][0],
                    self.items[1][0],
                    self.items[2][0],
                    self.items[3][0],
                ],
                vec![
                    self.items[0][1],
                    self.items[1][1],
                    self.items[2][1],
                    self.items[3][1],
                ],
                vec![
                    self.items[0][2],
                    self.items[1][2],
                    self.items[2][2],
                    self.items[3][2],
                ],
                vec![
                    self.items[0][3],
                    self.items[1][3],
                    self.items[2][3],
                    self.items[3][3],
                ],
            ],
            modified: false,
        };
    }

    fn add_random(&mut self, is_override: bool) {
        if self.modified || is_override {
            loop {
                let x = rand::thread_rng().gen_range(0..3);
                let y = rand::thread_rng().gen_range(0..3);
                let mut item = if self.items[x][y] == 0 {
                    let rng = rand::thread_rng().gen_range(0..100);
                    self.items[x][y] = [2, 4][if rng < 80 { 0 } else { 1 }];
                    break;
                };
            }
            self.modified = false;
        }
    }
}

impl Board {
    pub fn new() -> Self {
        let mut matrix = Self {
            items: vec![vec![0; 4]; 4],
            modified: false,
        };
        matrix.add_random(true);
        matrix.add_random(true);

        return matrix;
    }

    pub fn shift_right(&mut self) {
        self.modified = self
            .items
            .iter_mut()
            .map(|mut row| to_right(&mut row))
            .collect::<Vec<bool>>()
            .contains(&true);

        self.add_random(false);
    }

    pub fn shift_left(&mut self) {
        self.modified = self
            .items
            .iter_mut()
            .map(|mut row| to_left(&mut row))
            .collect::<Vec<bool>>()
            .contains(&true);

        self.add_random(false);
    }

    pub fn shift_up(&mut self) {
        let transposed = &mut self.transposed();
        self.modified = transposed
            .items
            .iter_mut()
            .map(|mut row| to_left(&mut row))
            .collect::<Vec<bool>>()
            .contains(&true);
        self.items = transposed.transposed().items;
        self.add_random(false);
    }

    pub fn shift_down(&mut self) {
        let transposed = &mut self.transposed();
        self.modified = transposed
            .items
            .iter_mut()
            .map(|mut row| to_right(&mut row))
            .collect::<Vec<bool>>()
            .contains(&true);
        self.items = transposed.transposed().items;
        self.add_random(false);
    }

    pub fn print(&mut self) {
        for row in self.items.iter() {
            println!("{:?}", row);
        }
        println!();
    }
}

fn to_right(row: &mut [i16]) -> bool {
    let mut modified = false;
    if align_to_right(row) {
        modified = true;
    }
    if join_pairs(row) {
        modified = true;
    }
    if align_to_right(row) {
        modified = true;
    }
    return modified;
}

fn to_left(row: &mut [i16]) -> bool {
    let mut modified = false;
    if align_to_left(row) {
        modified = true;
    }
    if join_pairs_left(row) {
        modified = true;
    }
    if align_to_left(row) {
        modified = true;
    }
    return modified;
}

fn align_to_right(row: &mut [i16]) -> bool {
    let mut modified = false;
    for i in (0..row.len()).rev() {
        if i == 0 {
            break;
        }
        if row[i] == 0 && row[i - 1] != 0 {
            row[i] = row[i - 1];
            row[i - 1] = 0;
            align_to_right(row);
            modified = true;
        }
    }
    return modified;
}

fn align_to_left(row: &mut [i16]) -> bool {
    let mut modified = false;
    for i in 0..row.len() - 1 {
        if i == row.len() - 1 {
            break;
        }
        if row[i] == 0 && row[i + 1] != 0 {
            row[i] = row[i + 1];
            row[i + 1] = 0;
            align_to_left(row);
            modified = true;
        }
    }
    return modified;
}

fn join_pairs(row: &mut [i16]) -> bool {
    let mut modified = false;
    for i in (0..row.len()).rev() {
        if i == 0 {
            break;
        }
        if row[i] != 0 && row[i] == row[i - 1] {
            modified = true;
            row[i] = row[i] + row[i - 1];
            row[i - 1] = 0;
        }
    }
    return modified;
}

fn join_pairs_left(row: &mut [i16]) -> bool {
    let mut modified = false;
    for i in 0..row.len() {
        if i == row.len() {
            break;
        }
        if row[i] != 0 && row[i] == row[i + 1] {
            modified = true;
            row[i] = row[i] + row[i + 1];
            row[i + 1] = 0;
        }
    }
    return modified;
}
