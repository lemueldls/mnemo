use std::fs;

use criterion::{Criterion, criterion_group, criterion_main};
use mnemo_wasm::{renderer::paged::items::render_by_items, state::TypstState};

fn benchmark_renderers(c: &mut Criterion) {
    let filename = String::from("math.typ");
    let text = fs::read_to_string(&filename).unwrap();

    let mut state = TypstState::default();

    state.install_font(
        include_bytes!("../../../public/fonts/new-cm/otf/NewCMMath-Book.otf").to_vec(),
    );
    state.install_font(
        include_bytes!("../../../public/fonts/new-cm/otf/NewCMMath-Regular.otf").to_vec(),
    );

    let id = state.create_file_id(filename);
    state.insert_source(&id, text.clone());

    let prelude = "";

    // c.bench_function("render by chunk", |b| {
    //     b.iter(|| render_by_chunk(&id, &text, prelude, &mut state))
    // });

    c.bench_function("render by items", |b| {
        b.iter(|| render_by_items(&id, &text, prelude, &mut state))
    });
}

criterion_group!(benches, benchmark_renderers);
criterion_main!(benches);
