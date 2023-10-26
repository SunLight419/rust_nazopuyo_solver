use crate::naive_next_puyo::NaiveNextPuyo;

#[derive(Debug, Clone)]
pub struct NazopuyoInfo {
    pub chain: u32,
    pub next: NaiveNextPuyo,
}