use std::fmt;
use crate::field::Field;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Field1D {
    value: [u8; 78],
}

impl fmt::Display for Field1D {
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

impl Field for Field1D {
    fn new() -> Self
        where
            Self: Sized {
        Self { value: [0; 78] }
    }

    fn from_u8(value: [[u8; 13]; 6]) -> Self {
        let mut v = Vec::with_capacity(78);
        for j in 0..6 {
            for i in 0..13 {
                v.push(value[j][i]);
            }
        }

        Self {
            value: v.try_into().expect("failed to initialize from u8")
        }
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

    #[inline]
    fn get(&self, y: usize, x: usize) -> u8 {
        self.value[y + x*13]
    }

    #[inline]
    fn set(&mut self, y: usize, x: usize, v: u8) {
        self.value[y + x*13] = v;
    }

    #[inline]
    fn is_alive(&self) -> bool {
        self.value[27] == 0
    }

    #[inline]
    fn is_empty(&self) -> bool {
        !self.value.iter().any(|&x| x != 0)
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

    
}