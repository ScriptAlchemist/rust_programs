use criterion::{black_box, criterion_group, criterion_main, Criterion};
use benchmark_test::{get_first_word_passing_ownership_of_string, get_first_word_borrowing_string, get_first_word_borrowing_slice, get_first_word_borrowing_slice_v2, get_first_word_byte_loop_borrowing_slice};

const LOREN_IPSUM: &str = include_str!("../lorem_ipsum.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    let lorem_ipsum = LOREN_IPSUM.to_string();

    c.bench_function("Get First Word Passing Ownership of String", |b| b.iter(|| get_first_word_passing_ownership_of_string(black_box(LOREN_IPSUM.to_string()))));
    c.bench_function("Get First Word Borrowing String", |b| b.iter(|| get_first_word_borrowing_string(black_box(&lorem_ipsum))));
    c.bench_function("Get First Word Borrowing Slice", |b| b.iter(|| get_first_word_borrowing_slice(black_box(LOREN_IPSUM))));
    c.bench_function("Get First Word Borrowing Slice V2", |b| b.iter(|| get_first_word_borrowing_slice_v2(black_box(LOREN_IPSUM))));
    c.bench_function("Get First Word Byte Loop Borrowing Slice", |b| b.iter(|| get_first_word_byte_loop_borrowing_slice(black_box(LOREN_IPSUM))));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

