//! Shared benchmark inputs.
//!
//! The benchmark itself lives in `benches/text.rs`. Keeping the samples here lets the benchmark
//! and any future comparison binaries agree on the same text and font.

/// The single English font used by every benchmark.
pub const FONT_DATA: &[u8] = include_bytes!("../assets/ComicMono.ttf");

/// Text samples covering short UI labels, prose that wraps, and a long single line.
pub struct Sample {
    pub name: &'static str,
    pub text: &'static str,
    pub font_size: f32,
    pub max_width: Option<f32>,
}

pub const SAMPLES: &[Sample] = &[
    Sample {
        name: "label",
        text: "Hello, tinim!",
        font_size: 16.0,
        max_width: None,
    },
    Sample {
        name: "paragraph",
        text: "Text shaping, line breaking and glyph rasterization. \
               The quick brown fox jumps over the lazy dog. \
               A single English font is enough to compare layout and rasterization speed.",
        font_size: 18.0,
        max_width: Some(480.0),
    },
    Sample {
        name: "long_line",
        text: "The quick brown fox jumps over the lazy dog. 0123456789 \
               Sphinx of black quartz, judge my vow. Pack my box with five dozen liquor jugs.",
        font_size: 24.0,
        max_width: None,
    },
];
