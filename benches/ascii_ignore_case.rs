use ascii_case_ext::AsciiExt;
use criterion::{Bencher, BenchmarkId, Criterion};

fn ascii_contains_ignore_case(c: &mut Criterion) {
    let mut group = c.benchmark_group("Ascii contains (ignore case)");
    for tup in [
        ("Hello, WoRld!", "wOrlD"),
        ("Long very long", "notcontains"),
    ] {
        group.bench_with_input(
            BenchmarkId::new("std", tup.0),
            &tup,
            |b: &mut Bencher, (main, sub): &(&str, &str)| b.iter(|| contains_std(main, sub)),
        );
        group.bench_with_input(
            BenchmarkId::new("my", tup.0),
            &tup,
            |b: &mut Bencher, (main, sub): &(&str, &str)| b.iter(|| contains_my(main, sub)),
        );
    }
}

fn contains_std(main: &str, sub: &str) -> bool {
    main.to_ascii_lowercase()
        .contains(&sub.to_ascii_lowercase())
}

fn contains_my(main: &str, sub: &str) -> bool {
    main.contains_ignore_ascii_case(sub)
}

criterion::criterion_group!(benches, ascii_contains_ignore_case);
criterion::criterion_main!(benches);
