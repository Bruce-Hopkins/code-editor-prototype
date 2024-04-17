use criterion::{black_box, criterion_group, criterion_main, Criterion};
use highlighter::{highlighter::{Highlighter, HighlighterConfig} };

fn bench(c: &mut Criterion) {
//     let document = Document::open("./test-dir/src/main.rs").expect("Couldn't open file");
//     let document_string = document.to_string();
//     let config = HighlighterConfig::rust_config(&document_string);
//     let mut document = Buffer::new(document, config);

// // HighlighterConfig::rust_config(&document_string);
   
//     c.bench_function("Insert string", |b| b.iter(||  {
//         document.insert("Hello World".to_owned());
//     }));

    let mut group = c.benchmark_group("highlighter-group");
    let document_string = "pub fn delete(&mut self, start_byte: usize, end_byte: usize, start_pos: Point, end_pos: Point, content:&RopeSlice) {}".repeat(100).to_owned();

    let config = HighlighterConfig::rust_config(&document_string);
    group.bench_function("new-highlighter", |b| b.iter(|| {
        let mut highlighter = Highlighter::new(black_box(&config), black_box(0..usize::MAX), black_box(&document_string));
        let highlighter_iter = highlighter.highlighter_iter(); 
        for item in highlighter_iter {}
    }));

    group.finish();

}

// fn criterion_benchmark2(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }


criterion_group!(benches, bench);
criterion_main!(benches);