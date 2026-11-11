use caps_widget::{Lang, LockState};

const CAPS_ON_NUM_OFF: LockState = LockState {
    caps: true,
    num: false,
};

#[test]
fn pt_variants_use_portuguese() {
    for tag in ["pt-BR", "pt-PT", "pt", "PT-br", "pt_BR"] {
        assert_eq!(Lang::from_locale_tag(tag), Lang::Pt, "tag {tag:?}");
        assert_eq!(
            CAPS_ON_NUM_OFF.tooltip(Lang::from_locale_tag(tag)),
            "Caps Lock: ligado | Num Lock: desligado"
        );
    }
}

#[test]
fn other_or_unknown_tags_use_english() {
    for tag in ["en-US", "es-ES", "", "ptx", "port", "fr-pt"] {
        assert_eq!(Lang::from_locale_tag(tag), Lang::En, "tag {tag:?}");
        assert_eq!(
            CAPS_ON_NUM_OFF.tooltip(Lang::from_locale_tag(tag)),
            "Caps Lock: on | Num Lock: off"
        );
    }
}

#[test]
fn menu_labels_per_language() {
    let pt = Lang::Pt.strings();
    assert_eq!(pt.start_with_windows, "Iniciar com o Windows");
    assert_eq!(pt.exit, "Sair");

    let en = Lang::En.strings();
    assert_eq!(en.start_with_windows, "Start with Windows");
    assert_eq!(en.exit, "Exit");
}
