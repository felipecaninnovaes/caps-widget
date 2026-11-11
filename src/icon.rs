//! Renders the tray icon in code: an "A" (Caps Lock) and a "1" (Num Lock),
//! each fully opaque when its key is on and faded when it is off.
//!
//! Drawing at runtime instead of shipping `.ico` files lets the icon match any
//! DPI-dependent size and keeps every variant testable.

use crate::state::LockState;
use crate::theme::Theme;

const GLYPH_W: u32 = 5;
const GLYPH_H: u32 = 7;
/// Two glyphs plus a one-column gap.
const GRID_W: u32 = GLYPH_W * 2 + 1;

/// 5×7 bitmaps, one row per byte, most significant of the low 5 bits = leftmost.
const GLYPH_A: [u8; 7] = [
    0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
];
const GLYPH_1: [u8; 7] = [
    0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
];

const ALPHA_ON: u8 = 255;
/// About 30% opacity: still visible, clearly "off".
const ALPHA_OFF: u8 = 77;

fn foreground(theme: Theme) -> [u8; 3] {
    match theme {
        Theme::Dark => [255, 255, 255],
        Theme::Light => [32, 32, 32],
    }
}

/// Renders a `size`×`size` icon as straight (non-premultiplied) RGBA, row-major
/// from the top-left. Transparent outside the glyphs.
pub fn render(state: LockState, theme: Theme, size: u32) -> Vec<u8> {
    let mut rgba = vec![0u8; (size * size * 4) as usize];

    // Integer scaling keeps strokes crisp; glyphs may be taller than wide
    // (up to one extra step) to use the square's height.
    let sx = (size / GRID_W).max(1);
    let sy = (size / GLYPH_H).clamp(1, sx + 1);
    let width = GRID_W * sx;
    let height = GLYPH_H * sy;
    let left = size.saturating_sub(width) / 2;
    let top = size.saturating_sub(height) / 2;

    let [r, g, b] = foreground(theme);
    let glyphs = [
        (&GLYPH_A, state.caps, 0),
        (&GLYPH_1, state.num, GLYPH_W + 1),
    ];
    for (bitmap, on, col0) in glyphs {
        let alpha = if on { ALPHA_ON } else { ALPHA_OFF };
        for (row, bits) in bitmap.iter().enumerate() {
            for col in 0..GLYPH_W {
                if bits & (1 << (GLYPH_W - 1 - col)) == 0 {
                    continue;
                }
                let x0 = left + (col0 + col) * sx;
                let y0 = top + row as u32 * sy;
                for y in y0..(y0 + sy).min(size) {
                    for x in x0..(x0 + sx).min(size) {
                        let i = ((y * size + x) * 4) as usize;
                        rgba[i..i + 4].copy_from_slice(&[r, g, b, alpha]);
                    }
                }
            }
        }
    }
    rgba
}
