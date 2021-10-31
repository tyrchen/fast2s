#[macro_use]
extern crate criterion;

use criterion::Criterion;
use fast2s::{convert, replace};

fn criterion_benchmark(c: &mut Criterion) {
    // as character_converter is too slow I have to change to sample size to 10
    let mut g = c.benchmark_group("sample 10");
    g.sample_size(10);

    let zht = include_str!("math_zht.txt");
    let zhc = include_str!("math_zhc.txt");
    let en = include_str!("math_en.txt");
    let mut zht1 = zht.to_string();
    let mut zhc1 = zhc.to_string();
    let mut en1 = en.to_string();
    let opencc = opencc_rust::OpenCC::new(opencc_rust::DefaultConfig::T2S).unwrap();
    let converter = character_converter::CharacterConverter::new();

    let convert_tests = [("t2s: zht", zht), ("t2s: zhc", zhc), ("t2s: en", en)];
    let mut replace_tests = [
        ("t2s replace: zht", &mut zht1),
        ("t2s replace: zhc", &mut zhc1),
        ("t2s replace: en", &mut en1),
    ];

    for (name, data) in convert_tests.iter() {
        g.bench_function(*name, |b| {
            b.iter(|| {
                convert(data);
            });
        });

        let id = format!("opencc {}", name);
        g.bench_function(&id, |b| {
            b.iter(|| {
                opencc.convert(data);
            });
        });

        let id = format!("simplet2s {}", name);
        g.bench_function(&id, |b| {
            b.iter(|| {
                simplet2s::convert(data);
            });
        });

        let id = format!("character_converter {}", name);
        g.bench_function(&id, |b| {
            b.iter(|| {
                converter.traditional_to_simplified(data);
            });
        });
    }

    for (name, data) in replace_tests.iter_mut() {
        g.bench_function(*name, |b| {
            b.iter(|| {
                replace(data);
            });
        });
    }

    g.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
