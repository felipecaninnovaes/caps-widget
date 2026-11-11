//! Renders the tray icon in code: an "A" (Caps Lock) and a "1" (Num Lock),
//! each fully opaque when its key is on and faded when it is off.
//!
//! Drawing at runtime instead of shipping `.ico` files lets the icon match any
//! DPI-dependent size and keeps every variant testable.

use crate::state::LockState;
use crate::theme::Theme;

/// A bitmap font with just the two glyphs the icon needs. Each row is a
/// bit mask whose most significant of the low `width` bits is the leftmost pixel.
struct Font {
    width: u32,
    a: &'static [u16],
    one: &'static [u16],
}

impl Font {
    fn height(&self) -> u32 {
        self.a.len() as u32
    }

    /// Two glyphs plus a one-column gap.
    fn grid_width(&self) -> u32 {
        self.width * 2 + 1
    }
}

/// Bold 7×10: legible at the common 16–20 px tray sizes.
const BOLD_7X10: Font = Font {
    width: 7,
    a: &[
        0b0011100, 0b0110110, 0b1100011, 0b1100011, 0b1100011, 0b1111111, 0b1100011, 0b1100011,
        0b1100011, 0b1100011,
    ],
    one: &[
        0b0001100, 0b0011100, 0b0111100, 0b0001100, 0b0001100, 0b0001100, 0b0001100, 0b0001100,
        0b0001100, 0b0111111,
    ],
};

/// Classic 5×7: fallback for tiny sizes and a better fit at some larger ones.
const THIN_5X7: Font = Font {
    width: 5,
    a: &[
        0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
    ],
    one: &[
        0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
    ],
};

const FONTS: [&Font; 2] = [&BOLD_7X10, &THIN_5X7];

const ALPHA_ON: u8 = 255;
/// About 30% opacity: still visible, clearly "off".
const ALPHA_OFF: u8 = 77;

fn foreground(theme: Theme) -> [u8; 3] {
    match theme {
        Theme::Dark => [255, 255, 255],
        Theme::Light => [32, 32, 32],
    }
}

/// The font and integer scale giving the tallest glyphs that fit in `size`.
/// The same scale on both axes keeps glyphs crisp and in proportion.
fn layout(size: u32) -> (&'static Font, u32) {
    FONTS
        .iter()
        .filter_map(|font| {
            let scale = (size / font.grid_width()).min(size / font.height());
            (scale > 0).then_some((*font, scale))
        })
        // `max_by_key` keeps the last maximum; reversing prefers the bold font on ties.
        .rev()
        .max_by_key(|(font, scale)| font.height() * scale)
        .unwrap_or((&THIN_5X7, 1))
}

/// Renders a `size`×`size` icon as straight (non-premultiplied) RGBA, row-major
/// from the top-left. Transparent outside the glyphs.
pub fn render(state: LockState, theme: Theme, size: u32) -> Vec<u8> {
    let mut rgba = vec![0u8; (size * size * 4) as usize];

    let (font, scale) = layout(size);
    let left = size.saturating_sub(font.grid_width() * scale) / 2;
    let top = size.saturating_sub(font.height() * scale) / 2;

    let [r, g, b] = foreground(theme);
    let glyphs = [
        (font.a, state.caps, 0),
        (font.one, state.num, font.width + 1),
    ];
    for (bitmap, on, col0) in glyphs {
        let alpha = if on { ALPHA_ON } else { ALPHA_OFF };
        for (row, bits) in bitmap.iter().enumerate() {
            for col in 0..font.width {
                if bits & (1 << (font.width - 1 - col)) == 0 {
                    continue;
                }
                let x0 = left + (col0 + col) * scale;
                let y0 = top + row as u32 * scale;
                for y in y0..(y0 + scale).min(size) {
                    for x in x0..(x0 + scale).min(size) {
                        let i = ((y * size + x) * 4) as usize;
                        rgba[i..i + 4].copy_from_slice(&[r, g, b, alpha]);
                    }
                }
            }
        }
    }
    rgba
}
