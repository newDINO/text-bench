//! Runs one layout sample in a loop for profiling.
//!
//! ```text
//! cargo run --release --example layout_loop -- paragraph 500
//! ```

use std::hint::black_box;
use std::time::Instant;

use text_bench::{FONT_DATA, SAMPLES};
use tinim_text::{TextContext, TextStyle};

fn main() {
    let mut args = std::env::args().skip(1);
    let name = args.next().unwrap_or_else(|| "paragraph".to_owned());
    let iterations: usize = args
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(1000);

    let sample = SAMPLES
        .iter()
        .find(|sample| sample.name == name)
        .unwrap_or_else(|| panic!("unknown sample {name:?}"));
    let style = TextStyle {
        font_size: sample.font_size,
        ..Default::default()
    };

    let mut text = TextContext::new();
    text.add_font(FONT_DATA.to_vec()).unwrap();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(text.layout(
            black_box(sample.text),
            black_box(&style),
            sample.max_width,
        ));
    }
    let elapsed = start.elapsed();

    println!(
        "{name}: {iterations} iterations in {elapsed:?} ({:.3} µs/layout)",
        elapsed.as_secs_f64() * 1e6 / iterations as f64
    );
}
