use crate::field::Field;
use std::fmt;
use colored::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct FieldNaiveBit {
    value: [u64; 6],
}

impl fmt::Display for FieldNaiveBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..13 {
            for j in 0..6 {
                let s = match self.get(i, j) {
                    1 => "@".color("white"),
                    2 => "2".color("red"),
                    3 => "3".color("blue"),
                    4 => "4".color("green"),
                    5 => "5".color("yellow"),
                    6 => "6".color("magenta"),
                    _ => "-".color("white"),
                };
                write!(f, "{:2} ", s)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Field for FieldNaiveBit {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            value: [0, 0, 0, 0, 0, 0],
        }
    }

    fn from_u8(value: [[u8; 13]; 6]) -> Self {
        let mut res = Self::new();
        for j in 0..6 {
            for i in 0..13 {
                res.set(i, j, value[j][i]);
            }
        }

        res
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
    fn set(&mut self, y: usize, x: usize, v: u8) {
        let mask = 7u64 << (y * 3);
        self.value[x] = self.value[x] & !mask | ((v as u64) << (y * 3));
    }

    #[inline]
    fn get(&self, y: usize, x: usize) -> u8 {
        let mask = 7u64 << (y * 3);
        ((self.value[x] & mask) >> (y * 3)) as u8
    }

    #[inline]
    fn fall(&mut self) {
        for j in 0..6 {
            if self.value[j] == 0 {
                continue;
            }
            let mut res = 0u64;
            let mut mask = 0b111u64;
            for _ in 0..13 {
                if self.value[j] & mask == 0 {
                    res <<= 3;
                } else {
                    res |= self.value[j] & mask;
                }
                mask <<= 3;
            }
            self.value[j] = res;
        }
    }

    #[inline]
    fn is_alive(&self) -> bool {
        (self.value[2] & 0b111000) == 0
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.value.iter().all(|x| *x == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_test() {
        let mut field = FieldNaiveBit::new();
        field.value[0] = 0b_100_011_010_001_000_111_110_101_100_011_010_001_000;
        println!("{}", field.value[0]);
        assert_eq!(field.get(0, 0), 0);
        assert_eq!(field.get(1, 0), 1);
        assert_eq!(field.get(2, 0), 2);
        assert_eq!(field.get(3, 0), 3);
        assert_eq!(field.get(4, 0), 4);
        assert_eq!(field.get(5, 0), 5);
        assert_eq!(field.get(6, 0), 6);
        assert_eq!(field.get(7, 0), 7);
        assert_eq!(field.get(8, 0), 0);
        assert_eq!(field.get(9, 0), 1);
        assert_eq!(field.get(10, 0), 2);
        assert_eq!(field.get(11, 0), 3);
        assert_eq!(field.get(12, 0), 4);
    }

    #[test]
    fn set_test() {
        let mut field = FieldNaiveBit::new();
        for i in 0..13 {
            field.set(i, 0, (i % 8) as u8);
        }
        println!("{}", field.value[0]);
        assert_eq!(field.get(0, 0), 0);
        assert_eq!(field.get(1, 0), 1);
        assert_eq!(field.get(2, 0), 2);
        assert_eq!(field.get(3, 0), 3);
        assert_eq!(field.get(4, 0), 4);
        assert_eq!(field.get(5, 0), 5);
        assert_eq!(field.get(6, 0), 6);
        assert_eq!(field.get(7, 0), 7);
        assert_eq!(field.get(8, 0), 0);
        assert_eq!(field.get(9, 0), 1);
        assert_eq!(field.get(10, 0), 2);
        assert_eq!(field.get(11, 0), 3);
        assert_eq!(field.get(12, 0), 4);
    }

    #[test]
    fn fall_test() {
        let mut field = FieldNaiveBit::new();
        field.value[0] = 0b_100_011_010_001_000_111_110_101_100_011_010_001_000;
        let expected = 0b_100_011_010_001_111_110_101_100_011_010_001_000_000_u64;
        field.fall();
        println!("{:b}", field.value[0]);
        println!("{:b}", expected);
        assert_eq!(field.value[0], expected);

        let mut field = FieldNaiveBit::new();
        field.value[0] = 0b_000_011_010_001_000_111_110_101_100_011_010_001_000;
        let expected = 0b_011_010_001_111_110_101_100_011_010_001_000_000_000_u64;
        field.fall();
        println!("{:b}", field.value[0]);
        println!("{:b}", expected);
        assert_eq!(field.value[0], expected);
    
    }
}
