use rust_nazopuyo_solver::field::{Field, self};
use rust_nazopuyo_solver::field1d::Field1D;
use rust_nazopuyo_solver::field_naive_bit::FieldNaiveBit;
use rust_nazopuyo_solver::naive_field::{NaiveField, kenny_bench};
use rust_nazopuyo_solver::nazopuyo_solver::chain_5depth;

fn main() {
    //kenny_bench();
    use std::time::Instant;
    let s = Instant::now();
    //rust_nazopuyo_solver::nazopuyo_solver::chain_6depth();
    rust_nazopuyo_solver::nazopuyo_solver::multi_test();
    let e = Instant::now();
    println!("{:?}", (e - s));

    // rust_nazopuyo_solver::field::kenny_bench::<NaiveField>();
    // rust_nazopuyo_solver::field::kenny_bench::<Field1D>();
    // rust_nazopuyo_solver::field::kenny_bench::<FieldNaiveBit>();
}
