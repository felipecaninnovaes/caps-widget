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
