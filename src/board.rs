use rand::{self, Rng};
use std::i16;

#[derive(Debug)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

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
                if self.items[x][y] == 0 {
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

    pub fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::UP => {
                let transposed = &mut self.transposed();
                self.modified = shift_rows(transposed, &to_left);
                self.items = transposed.transposed().items;
                self.add_random(false);
            }
            Direction::DOWN => {
                let transposed = &mut self.transposed();
                self.modified = shift_rows(transposed, &to_right);
                self.items = transposed.transposed().items;
                self.add_random(false);
            }
            Direction::RIGHT => {
                self.modified = shift_rows(self, &to_right);
                self.add_random(false);
            }
            Direction::LEFT => {
                self.modified = shift_rows(self, &to_left);

                self.add_random(false);
            }
        };
    }
}

fn shift_rows(board: &mut Board, f: &dyn Fn(&mut [i16]) -> bool) -> bool {
    return board
        .items
        .iter_mut()
        .map(|it| f(it))
        .collect::<Vec<bool>>()
        .contains(&true);
}

fn align_row(row: &mut [i16], f: &dyn Fn(&mut [i16]) -> bool) -> bool {
    return f(row);
}

fn to_right(row: &mut [i16]) -> bool {
    return [
        align_row(row, &align_to_right),
        align_row(row, &join_pairs),
        align_row(row, &align_to_right),
    ]
    .contains(&true);
}

fn to_left(row: &mut [i16]) -> bool {
    return [
        align_row(row, &align_to_left),
        align_row(row, &join_pairs_left),
        align_row(row, &align_to_left),
    ]
    .contains(&true);
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
        if i == row.len() - 1 {
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
