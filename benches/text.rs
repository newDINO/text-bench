//! Layout and glyph rasterization benchmark: `tinim-text` vs `glyph_brush_layout` / `ab_glyph`.
//!
//! Run with:
//!
//! ```text
//! cargo bench
//! ```
//!
//! Every benchmark uses the same single bundled English font, so differences come from the text
//! stack rather than from font coverage or fallback.

use std::hint::black_box;

use ab_glyph::{Font, FontArc, GlyphId};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use glyph_brush_layout::{
    FontId, GlyphPositioner, Layout, SectionGeometry, SectionText, ab_glyph::PxScale,
};
use text_bench::{FONT_DATA, SAMPLES};
use tinim_text::{TextStyle, rasterize};

/// Pixel size used for the rasterization benchmark.
const RASTERIZE_PPEM: f32 = 24.0;

/// Glyphs rasterized in every iteration, derived from the font's character map.
fn rasterize_glyph_ids(font: &FontArc) -> Vec<u32> {
    let mut ids: Vec<u32> = "The quick brown fox jumps over the lazy dog 0123456789!?"
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| font.glyph_id(ch).0 as u32)
        .filter(|id| *id != 0)
        .collect();
    ids.sort_unstable();
    ids.dedup();
    ids
}

fn bench_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("layout");
    group.sample_size(50);

    let mut tinim = tinim_text::TextContext::new();
    tinim.add_font(FONT_DATA.to_vec()).unwrap();

    let glyph_font = FontArc::try_from_slice(FONT_DATA).unwrap();
    let glyph_fonts = [glyph_font];

    for sample in SAMPLES {
        let style = TextStyle {
            font_size: sample.font_size,
            ..Default::default()
        };

        group.bench_with_input(
            BenchmarkId::new("tinim-text", sample.name),
            sample,
            |b, sample| {
                b.iter(|| {
                    black_box(tinim.layout(
                        black_box(sample.text),
                        black_box(&style),
                        sample.max_width,
                    ))
                })
            },
        );

        let geometry = SectionGeometry {
            screen_position: (0.0, 0.0),
            bounds: (sample.max_width.unwrap_or(f32::INFINITY), f32::INFINITY),
        };
        let section = SectionText {
            text: sample.text,
            scale: PxScale::from(sample.font_size),
            font_id: FontId(0),
        };

        group.bench_with_input(
            BenchmarkId::new("glyph_brush_layout", sample.name),
            sample,
            |b, _| {
                b.iter(|| {
                    black_box(Layout::default().calculate_glyphs(
                        black_box(&glyph_fonts),
                        black_box(&geometry),
                        black_box(&[section]),
                    ))
                })
            },
        );
    }

    group.finish();
}

fn bench_rasterize(c: &mut Criterion) {
    let mut group = c.benchmark_group("rasterize");
    group.sample_size(50);

    let glyph_font = FontArc::try_from_slice(FONT_DATA).unwrap();
    let glyph_ids = rasterize_glyph_ids(&glyph_font);
    group.throughput(Throughput::Elements(glyph_ids.len() as u64));

    let mut tinim = tinim_text::TextContext::new();
    let font_id = tinim.add_font(FONT_DATA.to_vec()).unwrap();
    let face = tinim.font(font_id).unwrap();

    group.bench_function(BenchmarkId::new("tinim-text", glyph_ids.len()), |b| {
        b.iter(|| {
            let mut pixels = 0usize;
            for &glyph_id in &glyph_ids {
                if let Some(glyph) = rasterize(black_box(face), black_box(glyph_id), RASTERIZE_PPEM)
                {
                    pixels += glyph.pixels.len();
                }
            }
            black_box(pixels)
        })
    });

    group.bench_function(BenchmarkId::new("ab_glyph", glyph_ids.len()), |b| {
        b.iter(|| {
            let mut coverage = 0.0f32;
            for &glyph_id in &glyph_ids {
                let glyph = GlyphId(glyph_id as u16).with_scale(RASTERIZE_PPEM);
                if let Some(glyph) = glyph_font.outline_glyph(black_box(glyph)) {
                    glyph.draw(|_, _, value| coverage += value);
                }
            }
            black_box(coverage)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_layout, bench_rasterize);
criterion_main!(benches);
