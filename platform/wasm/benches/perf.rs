use std::{
    fs::{self, File},
    io::BufWriter,
    path::PathBuf,
};

use mnemo_wasm::{
    renderer::paged::{items::chunk_by_items, svg::render_svgs_by_items},
    state::TypstState,
    world::MnemoWorld,
};
use typst_syntax::Span;
use walkdir::WalkDir;

fn main() {
    let mut state = TypstState::default();

    state.install_font(
        include_bytes!("../../../public/fonts/maple/ttf/MapleMono-Regular.ttf").to_vec(),
    );

    state.install_font(
        include_bytes!("../../../public/fonts/new-cm/otf/NewCMMath-Book.otf").to_vec(),
    );
    state.install_font(
        include_bytes!("../../../public/fonts/new-cm/otf/NewCMMath-Regular.otf").to_vec(),
    );

    typst_timing::enable();

    snapshot(&mut state);
}

pub fn snapshot(state: &mut TypstState) {
    let root = PathBuf::from("benches");
    let walk = WalkDir::new(&root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            entry.file_type().is_file() && entry.path().extension().unwrap().to_str() == Some("typ")
        });

    for entry in walk {
        let path = entry.path();
        let mut text = fs::read_to_string(&path).unwrap();

        let relative_path = path.strip_prefix(&root).unwrap();

        let id = state.create_source_id(
            relative_path.to_string_lossy().to_string(),
            String::from("test"),
        );
        state.insert_source(&id, text.clone());

        let prelude = "";

        for _ in 0..8 {
            render_svgs_by_items(&id, &text, prelude, state);

            text += "\nlmfao\n";
        }

        let file = File::create(relative_path.with_extension("json"))
            .map_err(|e| format!("failed to create file: {e}"))
            .unwrap();
        let writer = BufWriter::with_capacity(1 << 20, file);

        let world = state.world();
        typst_timing::export_json(writer, |span| {
            resolve_span(world, Span::from_raw(span)).unwrap_or_else(|| ("unknown".to_string(), 0))
        })
        .unwrap();

        typst_timing::clear();
    }
}

fn resolve_span(world: &MnemoWorld, span: Span) -> Option<(String, u32)> {
    let id = span.id()?;
    let source = world.get_source(id)?;
    let range = source.range(span)?;
    let line = source.lines().byte_to_line(range.start)?;
    Some((format!("{id:?}"), line as u32 + 1))
}
