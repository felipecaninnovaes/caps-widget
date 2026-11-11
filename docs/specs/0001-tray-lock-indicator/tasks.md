# 0001 — Tarefas

Cada tarefa: um commit, `just check` verde ao terminar, um teste que a prova.

- [x] **T1** — Núcleo: idiomas, tooltip e rótulos do menu
  - Requisitos: R4, R5
  - Arquivos: `src/lib.rs`, `src/state.rs`, `src/i18n.rs`, `tests/lock_state.rs`, `tests/i18n.rs`
  - Teste: `i18n::pt_variants_use_portuguese`, `i18n::other_or_unknown_tags_use_english`, `i18n::menu_labels_per_language`, `lock_state::tooltip_*`
  - Verificar: `just test --test i18n`

- [x] **T2** — Núcleo: detector de mudança e intervalo de verificação
  - Requisitos: R2
  - Arquivos: `src/state.rs`, `tests/change_detector.rs`
  - Teste: `change_detector::emits_once_per_change`, `change_detector::repeated_reading_emits_nothing`, `change_detector::poll_interval_meets_latency_target`
  - Verificar: `just test --test change_detector`

- [x] **T3** — Núcleo: tema e render das 8 variantes de ícone
  - Requisitos: R1, R9
  - Arquivos: `src/theme.rs`, `src/icon.rs`, `tests/icon.rs`
  - Teste: `icon::caps_on_num_off_lights_only_a`, `icon::four_states_render_distinct`, `icon::themes_render_distinct`, `icon::unknown_theme_is_dark`
  - Verificar: `just test --test icon`

- [x] **T4** — Núcleo: regras de "Iniciar com o Windows"
  - Requisitos: R7
  - Arquivos: `src/autostart.rs`, `tests/autostart.rs`
  - Teste: `autostart::disabled_by_default`, `autostart::enable_then_disable`, `autostart::moved_exe_reads_as_disabled`, `autostart::idempotent`, `autostart::store_failure_keeps_disabled`
  - Verificar: `just test --test autostart`

- [x] **T5** — `windows-sys` + adaptador de teclado (sem hook) + validação em segundo plano
  - Requisitos: R2, R3
  - Arquivos: `Cargo.toml`, `Cargo.lock`, `src/platform/mod.rs`, `src/platform/keyboard.rs`, `examples/poll_keys.rs`, `tests/keyboard_keys.rs`, `tests/no_keyboard_hook.rs`
  - Teste: `keyboard_keys::queries_only_caps_and_num`, `no_keyboard_hook::source_has_no_keyboard_hook`; manual: `cargo run --example poll_keys` com outra janela em foco mostra cada troca de Caps/Num (gate do risco principal — falhou → parar e discutir)
  - Verificar: `just test --test keyboard_keys`

- [ ] **T6** — Adaptadores de idioma, tema e registro de autostart
  - Requisitos: R4, R7, R9
  - Arquivos: `src/platform/locale.rs`, `src/platform/theme.rs`, `src/platform/autostart_registry.rs`, `tests/autostart_registry.rs`, `tests/platform_smoke.rs`
  - Teste: `autostart_registry::round_trip_in_temp_key`, `platform_smoke::locale_tag_not_empty`, `platform_smoke::theme_reads_without_panic`
  - Verificar: `just test --test autostart_registry`

- [ ] **T7** — Bandeja: instância única, janela oculta, ícone, polling, `TaskbarCreated`, sem console
  - Requisitos: R1, R2, R8, R10, R11
  - Arquivos: `src/platform/instance.rs`, `src/platform/tray.rs`, `src/main.rs`
  - Teste: testes automáticos de T1–T6 continuam verdes; manual CA2, CA5, CA16, CA19, CA20
  - Verificar: `just check` + `cargo run --release` e roteiro manual

- [ ] **T8** — Menu (Iniciar com o Windows, Sair) e troca de tema ao vivo
  - Requisitos: R5, R6, R7, R9
  - Arquivos: `src/platform/tray.rs`
  - Teste: manual CA10, CA18; marcar/desmarcar autostart e conferir em `HKCU\…\Run`
  - Verificar: `just check` + roteiro manual

- [ ] **T9** — Docs e roteiro manual
  - Requisitos: R12 (medição) + todos os CA manuais
  - Arquivos: `docs/specs/0001-tray-lock-indicator/manual-test.md`, `docs/architecture.md`, `CHANGELOG.md`, `docs/specs/README.md`
  - Teste: roteiro executado; CA21 (memória/CPU) medido e registrado
  - Verificar: `just check`

## Notas de desvio
<!-- T#: o que mudou em relação ao plano e por quê -->
- T4: regras como funções livres `autostart::{is_enabled, set_enabled}` sobre `AutostartStore` (erro `io::Error`) em vez de um tipo `Autostart`; mesmo comportamento.
- T5: guarda `no_keyboard_hook` também proíbe `RegisterRawInputDevices` e `GetAsyncKeyState` e varre `examples/`. ADR 0004 escrito junto com a dependência. **Gate manual de `poll_keys` em segundo plano ainda não executado** (injeção de teclas pelo agente foi negada) — rodar antes do PR.
