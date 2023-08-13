use criterion::{criterion_group, criterion_main, Criterion};
use evalexpr::{
    eval, eval_boolean_with_context, eval_empty_with_context_mut, Context, HashMapContext,
};
use evaluatorrs::formulas::root_formula::RootFormula;
use evaluatorrs::formulas::Evaluate;
use evaluatorrs::variable_stores::{EmptyVariableStore, HashMapStore, SetVariable};
use std::hint::black_box;

fn singl_token_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_token");
    group.bench_function("evalexpr", |b| b.iter(|| black_box(eval("1"))));
    group.bench_function("evaluatorrs", |b| {
        b.iter(|| {
            black_box({
                let store = EmptyVariableStore;
                let a = RootFormula::parse("1");
                a.eval(&store);
            })
        })
    });
    group.finish();
}

fn singl_eval_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_variable");
    group.bench_function("evalexpr", |b| {
        b.iter(|| {
            black_box({
                let mut context = HashMapContext::new();
                eval_empty_with_context_mut("a = 5", &mut context).unwrap();
                context.get_value("a");
            })
        })
    });
    group.bench_function("evaluatorrs", |b| {
        b.iter(|| {
            black_box({
                let mut store = HashMapStore::new();
                store.set("a", RootFormula::parse("5"));
                let a = RootFormula::parse("a");
                a.eval(&store);
            })
        })
    });
    group.finish();
}

criterion_group!(benches, singl_token_bench, singl_eval_bench);
criterion_main!(benches);
