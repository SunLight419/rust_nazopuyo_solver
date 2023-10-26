use crate::field;

use super::field::Field;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct NaiveField {
    value: [[u8; 13]; 6],
}

impl fmt::Display for NaiveField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..13 {
            for j in 0..6 {
                write!(f, "{:2} ", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Field for NaiveField {
    fn new() -> Self {
        NaiveField {
            value: [[0; 13]; 6],
        }
    }

    fn from_u8(value: [[u8; 13]; 6]) -> Self {
        NaiveField { value }
    }

    fn from_char(value: [[char; 13]; 6]) -> Self {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(&'@', &1u8);
        map.insert(&'r', &2);
        map.insert(&'b', &3);
        map.insert(&'g', &4);
        map.insert(&'y', &5);
        map.insert(&'p', &6);
        let mut res = [[0u8; 13]; 6];
        for j in 0..6 {
            for i in 0..13 {
                if map.contains_key(&value[j][i]) {
                    res[j][i] = **map.get(&value[j][i]).unwrap();
                } else {
                    res[j][i] = 0;
                }
            }
        }

        Self::from_u8(res)
    }

    // fn from(v: [[u32; 13]; 6]) -> Self{
    //             NaiveField { value: v }
    // }
    #[inline]
    fn set(&mut self, y: usize, x: usize, v: u8) {
        self.value[x][y] = v
    }

    #[inline]
    fn get(&self, y: usize, x: usize) -> u8 {
        self.value[x][y]
    }

    fn fall(&mut self) {
        for j in 0..6 {
            let mut space = 0usize;
            for i in (0..13).rev() {
                if self.get(i, j) == 0 {
                    space += 1;
                    continue;
                }
                if space == 0 {
                    continue;
                }
                let a = self.get(i, j);
                let b = self.get(i + space, j);
                self.set(i, j, b);
                self.set(i + space, j, a);
            }
        }
    }

    // fn chain(&mut self) -> u32 {
    //     let mut chain_count = 0u32;
    //     let dy = [0i32, 1, 0, -1];
    //     let dx = [1i32, 0, -1, 0];

    //     loop {
    //         let mut chained = false;
    //         let mut vis = [[false; 6]; 13];

    //         let mut field = NaiveField { value: self.value };

    //         for i in 1..13 {
    //             for j in 0..6 {
    //                 if vis[i][j] {
    //                     continue;
    //                 }
    //                 vis[i][j] = true;
    //                 let color = self.get(i, j);
    //                 if color == 0 || color == 1 {
    //                     continue;
    //                 }
    //                 let mut que = VecDeque::new();
    //                 let mut connected = VecDeque::new();

    //                 que.push_back((i, j));
    //                 connected.push_back((i, j));

    //                 while !que.is_empty() {
    //                     let (y, x) = que.pop_front().unwrap();
    //                     for k in 0..4 {
    //                         let ny = y as i32 + dy[k];
    //                         let nx = x as i32 + dx[k];
    //                         if nx < 0
    //                             || ny < 0
    //                             || 6 <= nx
    //                             || 13 <= ny
    //                             || vis[ny as usize][nx as usize]
    //                             || ny == 0
    //                         {
    //                             continue;
    //                         }
    //                         let new_color = field.get(ny as usize, nx as usize);
    //                         if new_color != color || new_color == 0 || new_color == 1 {
    //                             continue;
    //                         }
    //                         vis[ny as usize][nx as usize] = true;
    //                         que.push_back((ny as usize, nx as usize));
    //                         connected.push_back((ny as usize, nx as usize));
    //                     }
    //                 }

    //                 if connected.len() >= 4 {
    //                     chained = true;
    //                     for (y, x) in connected {
    //                         field.set(y, x, 0);
    //                         for k in 0..4 {
    //                             let ny = y as i32 + dy[k];
    //                             let nx = x as i32 + dx[k];
    //                             if nx < 0
    //                                 || ny < 0
    //                                 || 6 <= nx
    //                                 || 13 <= ny
    //                                 || ny == 0
    //                                 || field.get(ny as usize, nx as usize) != 1
    //                             {
    //                                 continue;
    //                             }
    //                             field.set(ny as usize, nx as usize, 0);
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         if !chained {
    //             break;
    //         }
    //         chain_count += 1;
    //         self.value = field.value;
    //         self.fall();
    //     }

    //     chain_count
    // }

    #[inline]
    fn is_empty(&self) -> bool {
        !self.value.iter().any(|x| x.iter().any(|y| *y != 0))
    }

    #[inline]
    fn is_alive(&self) -> bool {
        self.get(1, 2) == 0
    }
}

pub fn static_dispatch_chain<T: Field>(field: &mut T) -> u32
where
    T: Clone,
{
    // field.chain()
    field::chain(field)
}

#[allow(unused)]
pub fn static_dispatch_c_kenny_bench() {
    use std::time::Instant;
    let count = 1_000_000;
    let mut fields = Vec::with_capacity(count);
    for _ in 0..count {
        let kenny = [
            [0u8, 5, 6, 5, 6, 3, 6, 6, 6, 5, 6, 6, 6],
            [0u8, 4, 4, 4, 3, 4, 3, 3, 5, 3, 5, 5, 5],
            [5u8, 4, 5, 5, 4, 5, 4, 4, 5, 4, 3, 3, 3],
            [6u8, 5, 6, 6, 5, 6, 5, 5, 4, 6, 4, 4, 4],
            [3u8, 6, 3, 3, 6, 3, 6, 6, 5, 4, 6, 6, 6],
            [3u8, 4, 3, 4, 4, 4, 3, 3, 4, 4, 5, 5, 5],
        ];
        let field = NaiveField { value: kenny };
        fields.push(field);
    }
    let t_start = Instant::now();

    for f in fields.iter_mut() {
        static_dispatch_chain(f);
    }

    let t_end = Instant::now();
    let ell = t_end - t_start;
    println!("{:?} {:?}", ell, ell / count as u32);
}

#[allow(unused)]
pub fn kenny_bench() {
    use crate::field;
    use std::time::Instant;
    let count = 1_000_000;
    let mut fields = Vec::with_capacity(count);
    for _ in 0..count {
        let kenny = [
            [0u8, 5, 6, 5, 6, 3, 6, 6, 6, 5, 6, 6, 6],
            [0u8, 4, 4, 4, 3, 4, 3, 3, 5, 3, 5, 5, 5],
            [5u8, 4, 5, 5, 4, 5, 4, 4, 5, 4, 3, 3, 3],
            [6u8, 5, 6, 6, 5, 6, 5, 5, 4, 6, 4, 4, 4],
            [3u8, 6, 3, 3, 6, 3, 6, 6, 5, 4, 6, 6, 6],
            [3u8, 4, 3, 4, 4, 4, 3, 3, 4, 4, 5, 5, 5],
        ];
        let field = NaiveField { value: kenny };
        fields.push(field);
    }
    let t_start = Instant::now();

    for f in fields.iter_mut() {
        //f.chain();
        field::chain(f);
    }

    let t_end = Instant::now();
    let ell = t_end - t_start;
    println!("{:?} {:?}", ell, ell / count as u32);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn fall() {
        let mut field = NaiveField::new();
        field.set(0, 0, 1);
        field.set(3, 0, 2);
        field.set(4, 0, 3);
        field.fall();
        let mut req_field = NaiveField::new();
        req_field.set(10, 0, 1);
        req_field.set(11, 0, 2);
        req_field.set(12, 0, 3);

        assert_eq!(field, req_field);
    }

    #[test]
    fn chain() {
        // g -> 3, b -> 4, y -> 5, p -> 6
        let kenny = [
            [0u8, 5, 6, 5, 6, 3, 6, 6, 6, 5, 6, 6, 6],
            [0u8, 4, 4, 4, 3, 4, 3, 3, 5, 3, 5, 5, 5],
            [5u8, 4, 5, 5, 4, 5, 4, 4, 5, 4, 3, 3, 3],
            [6u8, 5, 6, 6, 5, 6, 5, 5, 4, 6, 4, 4, 4],
            [3u8, 6, 3, 3, 6, 3, 6, 6, 5, 4, 6, 6, 6],
            [3u8, 4, 3, 4, 4, 4, 3, 3, 4, 4, 5, 5, 5],
        ];
        let mut field = NaiveField::new();
        field.value = kenny;
        // println!("{}", field);
        assert_eq!(field::chain(&mut field), 19);
    }

    #[test]
    fn chain_with_gohst() {
        let full = [
            [2u8, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            [2u8, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            [2u8, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            [2u8, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            [2u8, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            [2u8, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        ];
        let mut field = NaiveField { value: full };
        assert_eq!(field::chain(&mut field), 2);
    }

    use crate::field::get_top;
    #[test]
    fn get_top_test() {
        let mut field = NaiveField::new();
        assert_eq!(get_top(&field, 0), None);
        field.set(12, 0, 1);
        assert_eq!(get_top(&field, 0).unwrap(), 12);
        field.set(11, 0, 1);
        field.set(10, 0, 1);
        assert_eq!(get_top(&field, 0).unwrap(), 10);
    }

    #[test]
    fn chain_with_ojama() {
        let mut field = NaiveField::new();
        field.set(12, 0, 1);
        field.set(12, 1, 2);
        field.set(12, 2, 2);
        field.set(11, 0, 1);
        field.set(11, 1, 2);
        field.set(11, 2, 2);
        field::chain(&mut field);
        assert_eq!(field, NaiveField::new());
    }
}
