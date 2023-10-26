#[derive(Debug, Clone)]
pub struct NaiveNextPuyo {
    pub value: [[u8; 2]; 10],
    pub len: usize,
}

impl NaiveNextPuyo {
    pub fn new() -> Self {
        NaiveNextPuyo {
            value: [[0u8; 2]; 10],
            len: 0,
        }
    }

    pub fn from_vec(v: Vec<[u8; 2]>) -> Self {
        NaiveNextPuyo {
            len: v.len(),
            value: v.try_into().unwrap(),
        }
    }
}