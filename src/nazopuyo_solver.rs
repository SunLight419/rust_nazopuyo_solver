use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::hash::Hash;
use std::time::Duration;

use crate::field::{self, get_top, Field};
use crate::field1d::Field1D;
use crate::field_naive_bit::FieldNaiveBit;
use crate::naive_field::NaiveField;
// use crate::naive_next_puyo::NaiveNextPuyo;
use crate::nazopuyo_info::NazopuyoInfo;
// use crate::next_puyo::NexuPuyo;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self, ThreadId};

// use super::field::Field;

#[derive(Debug)]
pub struct Solver<F>
where
    F: Field + Clone + PartialEq + Eq + Hash + std::fmt::Display + Send + 'static,
{
    field: F, //Box<dyn Field>,
    info: NazopuyoInfo,
    hash: HashSet<F>,
}

impl<F> Solver<F>
where
    F: Field + Clone + PartialEq + Eq + Hash + std::fmt::Display + Send + 'static,
{
    fn new(field: F, info: NazopuyoInfo) -> Self {
        Self {
            field,
            info,
            hash: HashSet::new(),
        }
    }
    pub fn solve(&mut self) -> Option<F> {
        match self.dfs(self.field.clone(), self.info.clone(), 0) {
            Some(field) => Some(field),
            None => None,
        }
    }

    fn dfs(&mut self, mut field: F, info: NazopuyoInfo, depth: u32) -> Option<F> {
        if depth == info.next.len as u32 {
            let f2 = field.clone();
            // calclate sum of field.value, which is [[u8; 13]; 6];
            //let total = field.value.iter().map(|x| x.iter().sum::<u8>()).sum::<u8>();
            if field::chain(&mut field) == info.chain {
                // println!("{}", f2);
                //println!("{}", field);
                return Some(f2);
            } else {
                return None;
            }
        }

        let indicies = [2, 4, 3, 5, 1, 0];

        for j in indicies {
            for index in 0..2 {
                let y1 = match get_top(&field, j) {
                    Some(y) => match y {
                        0 => continue,
                        _ => y - 1,
                    },
                    None => 12,
                };
                field.set(y1, j, info.next.value[depth as usize][index]);
                for dx in 0..2 {
                    if j + dx >= 6 {
                        continue;
                    }
                    let y2 = match get_top(&field, j + dx) {
                        Some(y) => match y {
                            0 => {
                                // field.set(y1, j, 0);
                                continue;
                            }
                            _ => y - 1,
                        },
                        None => 12,
                    };
                    field.set(y2, j + dx, info.next.value[depth as usize][index ^ 1]);
                    if !self.hash.contains(&field)
                        && (depth + 1 == info.next.len as u32
                            || (field.is_alive() && !field::has_chain(&field)))
                    {
                        self.hash.insert(field.clone());
                        let res = self.dfs(field.clone(), info.clone(), depth + 1);
                        if res.is_some() {
                            return res;
                        }
                    }
                    field.set(y2, j + dx, 0);
                }
                field.set(y1, j, 0);
            }
        }

        None
    }

    pub fn solve_multi(&mut self) -> Option<F> {
        let (tx, rx) = mpsc::channel();

        let mut fields = vec![];
        let indicies = [2, 4, 3, 5, 1, 0];

        for j in indicies {
            for index in 0..2 {
                let y1 = match get_top(&self.field, j) {
                    Some(y) => match y {
                        0 => continue,
                        _ => y - 1,
                    },
                    None => 12,
                };
                self.field.set(y1, j, self.info.next.value[0][index]);
                for dx in 0..2 {
                    if j + dx >= 6 {
                        continue;
                    }
                    let y2 = match get_top(&self.field, j + dx) {
                        Some(y) => match y {
                            0 => {
                                // field.set(y1, j, 0);
                                continue;
                            }
                            _ => y - 1,
                        },
                        None => 12,
                    };
                    self.field
                        .set(y2, j + dx, self.info.next.value[0][index ^ 1]);
                    if !self.hash.contains(&self.field)
                        && self.field.is_alive()
                        && !field::has_chain(&self.field)
                    {
                        self.hash.insert(self.field.clone());
                        fields.push(self.field.clone());
                    }
                    self.field.set(y2, j + dx, 0);
                }
                self.field.set(y1, j, 0);
            }
        }

        println!("{} patterns!", fields.len());

        // for f in fields.iter() {
        //     println!("{}", f);
        // }
        let mut handles = Vec::new();
        let info2 = self.info.clone();
        for f in fields.into_iter() {
            let tx = tx.clone();
            let mut f2 = f.clone();
            let info3 = info2.clone();
            let mut hash2 = self.hash.clone();
            let handle = thread::spawn(move || {
                let res = parallel_dfs(&mut f2, info3, 1, hash2.borrow_mut());
                if res.is_some() {
                    println!("found!");
                    tx.send(res).ok();
                } else {
                    println!("No result for {:?}", thread::current().id());
                }
            });
            handles.push(handle);
            //thread::sleep(Duration::from_millis(1000));
        }

        // for handle in handles {
        //     handle.join().unwrap();
        // }
        drop(tx);

        rx.recv().ok().and_then(|opt| opt)
    }
}

fn parallel_dfs<F>(
    field: &mut F,
    info: NazopuyoInfo,
    depth: u32,
    hash: &mut HashSet<F>,
) -> Option<F>
where
    F: Field + Clone + PartialEq + Eq + Hash + std::fmt::Display + Send,
{
    if depth == info.next.len as u32 {
        let f2 = &mut field.clone();
        let f3 = field.clone();
        if field::chain(f2) == info.chain {
            return Some(f3);
        } else {
            return None;
        }
    }

    let indicies = [2, 4, 3, 5, 1, 0];

    for j in indicies {
        for index in 0..2 {
            let y1 = match get_top(field, j) {
                Some(y) => match y {
                    0 => continue,
                    _ => y - 1,
                },
                None => 12,
            };
            field.set(y1, j, info.next.value[depth as usize][index]);
            for dx in 0..2 {
                if j + dx >= 6 {
                    continue;
                }
                let y2 = match get_top(field, j + dx) {
                    Some(y) => match y {
                        0 => {
                            // field.set(y1, j, 0);
                            continue;
                        }
                        _ => y - 1,
                    },
                    None => 12,
                };
                field.set(y2, j + dx, info.next.value[depth as usize][index ^ 1]);

                if !hash.contains(&field)
                    && (depth + 1 == info.next.len as u32
                        || (field.is_alive() && !field::has_chain(field)))
                {
                    hash.insert(field.clone());
                    {
                        let res = parallel_dfs(field, info.clone(), depth + 1, hash.borrow_mut());
                        if res.is_some() {
                            return res;
                        }
                    }
                }
                field.set(y2, j + dx, 0);
            }
            field.set(y1, j, 0);
        }
    }
    None
}

#[allow(unused)]
pub fn chain_5depth() {
    let value = [
        [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5],
        [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 2],
        [0u8, 0, 0, 0, 5, 5, 3, 3, 4, 4, 5, 2, 2],
        [0u8, 0, 0, 0, 0, 5, 3, 2, 4, 5, 5, 3, 3],
        [0u8, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3, 5, 3],
        [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 5],
    ];
    let field = NaiveField::from_u8(value);
    println!("{}", field);
    let mut next = crate::naive_next_puyo::NaiveNextPuyo::new();
    next.len = 5;
    next.value[0][0] = 3;
    next.value[0][1] = 5;
    next.value[1][0] = 2;
    next.value[1][1] = 3;
    next.value[2][0] = 4;
    next.value[2][1] = 5;
    next.value[3][0] = 2;
    next.value[3][1] = 5;
    next.value[4][0] = 3;
    next.value[4][1] = 5;
    let info = NazopuyoInfo { chain: 10, next };
    let mut solver = Solver::new(field, info);
    let res = solver.solve().unwrap();
    println!("{}", res);
}

#[allow(unused)]
pub fn chain_6depth() {
    let value = [
        [
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ],
        [
            ' ', ' ', ' ', ' ', 'p', 'p', 'r', 'r', 'b', 'g', 'g', 'r', 'r',
        ],
        [
            ' ', ' ', ' ', ' ', 'p', 'b', 'b', 'r', 'b', 'b', 'g', 'y', 'r',
        ],
        [
            ' ', ' ', ' ', ' ', 'r', 'b', 'g', 'g', 'y', 'r', 'y', 'y', 'b',
        ],
        [
            ' ', ' ', ' ', ' ', 'r', 'r', 'g', 'y', 'y', 'r', 'r', 'b', 'b',
        ],
        [
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ],
    ];
    let field = FieldNaiveBit::from_char(value);
    println!("{}", field);
    let mut next = crate::naive_next_puyo::NaiveNextPuyo::new();
    next.len = 6;
    next.value[0][0] = 2;
    next.value[0][1] = 3;
    next.value[1][0] = 2;
    next.value[1][1] = 4;
    next.value[2][0] = 5;
    next.value[2][1] = 3;
    next.value[3][0] = 6;
    next.value[3][1] = 4;
    next.value[4][0] = 2;
    next.value[4][1] = 5;
    next.value[5][0] = 2;
    next.value[5][1] = 3;
    let info = NazopuyoInfo { chain: 12, next };
    let mut solver = Solver::new(field, info);
    let res = solver.solve().unwrap();
    println!("{}", res);
}

#[allow(unused)]
pub fn multi_test() {
    let value = [
        [
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ],
        [
            ' ', ' ', ' ', ' ', 'p', 'p', 'r', 'r', 'b', 'g', 'g', 'r', 'r',
        ],
        [
            ' ', ' ', ' ', ' ', 'p', 'b', 'b', 'r', 'b', 'b', 'g', 'y', 'r',
        ],
        [
            ' ', ' ', ' ', ' ', 'r', 'b', 'g', 'g', 'y', 'r', 'y', 'y', 'b',
        ],
        [
            ' ', ' ', ' ', ' ', 'r', 'r', 'g', 'y', 'y', 'r', 'r', 'b', 'b',
        ],
        [
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ],
    ];
    let field = FieldNaiveBit::from_char(value);
    println!("{}", field);
    let mut next = crate::naive_next_puyo::NaiveNextPuyo::new();
    next.len = 6;
    next.value[0][0] = 2;
    next.value[0][1] = 3;
    next.value[1][0] = 2;
    next.value[1][1] = 4;
    next.value[2][0] = 5;
    next.value[2][1] = 3;
    next.value[3][0] = 6;
    next.value[3][1] = 4;
    next.value[4][0] = 2;
    next.value[4][1] = 5;
    next.value[5][0] = 2;
    next.value[5][1] = 3;
    let info = NazopuyoInfo { chain: 12, next };
    let mut solver = Solver::new(field, info);
    let res = solver.solve_multi();
    println!("{}", res.unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::naive_next_puyo::NaiveNextPuyo;
    #[test]
    fn it_works() {
        let mut field = NaiveField::new();
        field.set(12, 1, 1);
        field.set(12, 2, 2);
        field.set(12, 3, 2);
        field.set(12, 4, 2);
        field.set(12, 5, 4);
        field.set(11, 1, 1);
        field.set(11, 2, 1);
        field.set(11, 3, 1);
        field.set(11, 4, 4);
        field.set(11, 5, 4);
        field.set(10, 3, 3);
        field.set(10, 4, 3);
        println!("{}", field);
        let mut next = NaiveNextPuyo::new();
        next.len = 2;
        next.value[0][0] = 3;
        next.value[0][1] = 4;
        next.value[1][0] = 3;
        next.value[1][1] = 2;
        let info = NazopuyoInfo { chain: 3, next };
        let mut solver = Solver::new(field, info);
        let res = solver.solve();
        assert!(res.is_some());
    }
}
