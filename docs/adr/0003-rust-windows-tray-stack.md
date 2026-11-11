# 0003 — Stack: Rust, só Windows, ícone na bandeja

- **Status:** aceito
- **Data:** 2026-11-11

## Contexto
O caps-widget é um utilitário de desktop pequeno que precisa ler o estado de Caps/Num Lock e mostrá-lo continuamente, consumindo o mínimo de memória e CPU. O público usa Windows; Linux (X11/Wayland) e macOS complicariam a leitura do estado sem benefício imediato. O projeto é open source (MIT) e distribuído, no futuro, como um executável único.

## Opções consideradas
- **Linguagem:** Rust · Go · TypeScript (Electron/Tauri) · Python. → **Rust**: binário único e leve, acesso direto e tipado à API Win32 (`windows-sys`), sem runtime.
- **Plataformas:** só Windows · Windows + Linux · multiplataforma. → **só Windows**; o núcleo puro permite adicionar plataformas depois atrás de `src/platform/`.
- **Apresentação:** ícone na bandeja · overlay/OSD · ambos. → **ícone na bandeja**; overlay pode ser feature futura.
- **CI:** tudo no devbox (Linux) · job Windows dedicado. → **job `check` em `windows-latest` sem devbox** (devbox não roda no Windows; lint/test no Linux não validariam o código Win32). Higiene (pre-commit, gitleaks) segue no devbox em Linux.
- **Distribuição/versionamento:** nenhum por enquanto · release-please · manual. → **nenhum por enquanto**.

## Decisão
1. Rust edição 2024, toolchain fixada em `rust-toolchain.toml` (1.97) com rustfmt e clippy; `Cargo.lock` versionado.
2. Contrato do `justfile`: `cargo fmt` / `cargo fmt --check` / `rustfmt` (só `.rs`) / `cargo clippy -D warnings` / `cargo test {{args}}` / `cargo build --release`.
3. Núcleo puro em `src/lib.rs`; Win32 isolado em `src/platform/`. Crates de plataforma (`windows-sys`, `tray-icon`) escolhidas na primeira spec.
4. Sem gate de cobertura; teste obrigatório por convenção.

## Consequências
- ✅ Executável pequeno, sem dependências de runtime; núcleo testável em qualquer SO.
- ✅ CI valida o target real (MSVC).
- ⚠️ No devbox/WSL só o núcleo é exercitado; o caminho Win32 só é verificado no Windows (local ou CI).
- ⚠️ O job `check` foge da regra "CI sempre via devbox" do ADR 0002 — exceção documentada em `.claude/rules/ci.md`.
- ⚠️ Suporte a outras plataformas exigirá novo ADR.
