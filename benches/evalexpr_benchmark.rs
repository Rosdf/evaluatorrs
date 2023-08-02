use criterion::{criterion_group, criterion_main, Criterion};
use eval_tmp::formulas::Evaluate;
use evalexpr::{eval, eval_empty_with_context_mut, Context, HashMapContext};

fn test1() {
    eval("1");
}

fn evalexpr_benchmark(c: &mut Criterion) {
    c.bench_function("evalexpr test", |b| b.iter(|| test1()));
}

criterion_group!(benches, evalexpr_benchmark);
criterion_main!(benches);
