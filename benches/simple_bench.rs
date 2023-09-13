use criterion::{criterion_group, criterion_main, Criterion};
use evalexpr::{eval, eval_empty_with_context_mut, Context, HashMapContext};
use evaluatorrs::formulas::Evaluate;
use evaluatorrs::formulas::RootFormula;
use evaluatorrs::formulas::Sin;
use evaluatorrs::function_stores::{EmptyFunctionStore, HashMapFunctionStore, RegisterParser};
use evaluatorrs::variable_stores::{EmptyVariableStore, HashMapVariableStore, SetVariable};
use std::hint::black_box;
use std::time::Duration;

fn singl_token_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_token");
    group.warm_up_time(Duration::from_secs(15));
    group.sample_size(10000);
    group.bench_function("evalexpr", |b| b.iter(|| black_box(eval("1"))));
    group.bench_function("evaluatorrs", |b| {
        b.iter(|| {
            black_box({
                let store = EmptyVariableStore;
                let a = RootFormula::parse("1", &EmptyFunctionStore).unwrap();
                a.eval(&store);
            })
        })
    });
    group.finish();
}

fn singl_eval_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_variable");
    group.warm_up_time(Duration::from_secs(15));
    group.sample_size(10000);
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
                let mut store = HashMapVariableStore::new();
                store.set("a", RootFormula::parse("5", &EmptyFunctionStore).unwrap());
                let a = RootFormula::parse("a", &EmptyFunctionStore).unwrap();
                a.eval(&store);
            })
        })
    });
    group.finish();
}

fn big_expression(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_variable");
    group.warm_up_time(Duration::from_secs(15));
    group.sample_size(10000);
    group.bench_function("evalexpr", |b| {
        b.iter(|| {
            black_box({
                eval("5*(45+6/math::sin(1))").unwrap();
            })
        })
    });
    group.bench_function("evaluatorrs", |b| {
        b.iter(|| {
            black_box({
                let mut function_store = HashMapFunctionStore::new();
                function_store.register::<Sin>();
                let a = RootFormula::parse("5*(45+6/sin(1))", &function_store).unwrap();
                a.eval(&EmptyVariableStore);
            })
        })
    });
    group.finish();
}

criterion_group!(benches, big_expression); //, singl_token_bench, singl_eval_bench);
criterion_main!(benches);
