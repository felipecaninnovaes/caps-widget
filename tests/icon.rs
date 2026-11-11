use std::collections::HashSet;

use caps_widget::{LockState, Theme, icon};

const SIZES: [u32; 5] = [16, 20, 24, 32, 48];

fn all_states() -> [LockState; 4] {
    [(false, false), (true, false), (false, true), (true, true)]
        .map(|(caps, num)| LockState { caps, num })
}

/// Highest alpha found in the columns `[from, to)`.
fn max_alpha(rgba: &[u8], size: u32, from: u32, to: u32) -> u8 {
    (0..size)
        .flat_map(|y| (from..to).map(move |x| ((y * size + x) * 4 + 3) as usize))
        .map(|i| rgba[i])
        .max()
        .unwrap_or(0)
}

#[test]
fn render_has_one_rgba_pixel_per_cell() {
    for size in SIZES {
        let rgba = icon::render(LockState::default(), Theme::Dark, size);
        assert_eq!(rgba.len(), (size * size * 4) as usize, "size {size}");
    }
}

#[test]
fn caps_on_num_off_lights_only_a() {
    let state = LockState {
        caps: true,
        num: false,
    };
    for size in SIZES {
        let rgba = icon::render(state, Theme::Dark, size);
        let half = size / 2;
        let a = max_alpha(&rgba, size, 0, half);
        let one = max_alpha(&rgba, size, half, size);
        assert_eq!(a, 255, "\"A\" should be fully lit at {size}px");
        assert!(one > 0, "\"1\" should still be drawn at {size}px");
        assert!(one < a / 2, "\"1\" should be dimmed at {size}px");
    }
}

#[test]
fn four_states_render_distinct() {
    for size in SIZES {
        let renders: HashSet<_> = all_states()
            .into_iter()
            .map(|s| icon::render(s, Theme::Dark, size))
            .collect();
        assert_eq!(renders.len(), 4, "size {size}");
    }
}

#[test]
fn themes_render_distinct() {
    for size in SIZES {
        let renders: HashSet<_> = all_states()
            .into_iter()
            .flat_map(|s| [Theme::Light, Theme::Dark].map(|t| icon::render(s, t, size)))
            .collect();
        assert_eq!(renders.len(), 8, "size {size}");
    }
}

#[test]
fn unknown_theme_is_dark() {
    assert_eq!(Theme::from_light_flag(None), Theme::Dark);
    assert_eq!(Theme::from_light_flag(Some(0)), Theme::Dark);
    assert_eq!(Theme::from_light_flag(Some(1)), Theme::Light);
    assert_eq!(Theme::default(), Theme::Dark);
}

/// Bounding box `(width, height)` of every non-transparent pixel.
fn ink_box(rgba: &[u8], size: u32) -> (u32, u32) {
    let inked: Vec<(u32, u32)> = (0..size)
        .flat_map(|y| (0..size).map(move |x| (x, y)))
        .filter(|&(x, y)| rgba[((y * size + x) * 4 + 3) as usize] > 0)
        .collect();
    let xs = inked.iter().map(|p| p.0);
    let ys = inked.iter().map(|p| p.1);
    (
        xs.clone().max().unwrap() - xs.min().unwrap() + 1,
        ys.clone().max().unwrap() - ys.min().unwrap() + 1,
    )
}

#[test]
fn glyphs_are_not_stretched() {
    let all_on = LockState {
        caps: true,
        num: true,
    };
    for size in SIZES {
        let (w, h) = ink_box(&icon::render(all_on, Theme::Dark, size), size);
        // Two glyphs side by side are clearly wider than tall (~1.5:1);
        // stretching them vertically pushes this towards or below 1.
        let ratio = w as f32 / h as f32;
        assert!(
            (1.3..=1.8).contains(&ratio),
            "{size}px: ink {w}x{h} (ratio {ratio:.2})"
        );
        assert!(w * 10 >= size * 6, "{size}px: ink only {w}px wide");
    }
}
