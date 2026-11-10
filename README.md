# text-bench

A small performance comparison for [`tinim-text`](../tinim-gui/crates/tinim-text). It measures:

- **layout** — shaping and line breaking with `tinim-text` vs `glyph_brush_layout`
- **rasterize** — glyph outline rasterization with `tinim-text` vs `ab_glyph`

Both use a single bundled English font (`assets/ComicMono.ttf`, copied from the tinim demo
assets) and the same text samples, so there is no fallback or multi-font work in either side.

## Run

```sh
cargo bench
```

Filter to one group:

```sh
cargo bench -- layout
cargo bench -- rasterize
```

Compare a change against a saved run:

```sh
cargo bench -- --save-baseline before
# edit something
cargo bench -- --baseline before
```

Criterion's HTML report is written to `target/criterion/report/index.html`.

## Samples

| name | what it covers |
|---|---|
| `label` | a short single line without wrapping |
| `paragraph` | several lines wrapped at a fixed width |
| `long_line` | a long unwrapped line with letters, digits and punctuation |

The rasterize benchmark uses the unique glyphs in an English pangram at 24 px.

## Notes

- Layout output is produced but immediately discarded through `black_box`; this measures the
  layout pipeline, not downstream mesh/atlas work.
- `tinim-text` outputs RGBA8 glyph bitmaps while `ab_glyph` emits coverage through a callback, so
  rasterization numbers compare the total outline → coverage work of each API, not byte-identical
  output.
- `glyph_brush_layout` has no configurable line height, so the samples use each library's default
  font metrics.
