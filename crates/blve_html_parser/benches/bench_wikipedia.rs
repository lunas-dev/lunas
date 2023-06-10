use blve_html_parser::Dom;
use criterion::{criterion_group, criterion_main, Criterion};

static HTML: &'static str = include_str!("./wikipedia-2020-12-21.html");

fn wikipedia(c: &mut Criterion) {
    c.bench_function("wikipedia", |b| b.iter(|| Dom::parse(HTML).unwrap()));
}

criterion_group!(benches, wikipedia);
criterion_main!(benches);
