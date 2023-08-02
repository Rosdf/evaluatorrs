use criterion::{criterion_group, criterion_main, Criterion};
use eval_tmp::formulas::base_formula::BaseFormula;
use eval_tmp::formulas::Evaluate;
use eval_tmp::variable_stores::{HashMapStore};

fn test1() {
    let store = HashMapStore::new();
    let mut a = BaseFormula::new("1");
    a.eval(&store);
}

fn evaluatorrs_benchmark(c: &mut Criterion) {
    c.bench_function("evaluatorrs test", |b| b.iter(|| test1()));
}

fn evalexpr_benchmark(c: &mut Criterion) {
    c.bench_function("evalexpr test", |b| b.iter(|| test1()));
}

criterion_group!(benches, evaluatorrs_benchmark);
criterion_main!(benches);
