# 0001 — Plano técnico

- **Spec:** [spec.md](spec.md)
- **Status:** aprovado <!-- rascunho | aprovado -->

## Abordagem
Separação estrita **núcleo puro × adaptador Win32**, seguindo `src/lib.rs` (núcleo sem SO) e `.claude/rules/rust.md`.

**Núcleo (`src/`, testável em qualquer SO)** — toda decisão fica aqui:
- `state`: `LockState` (já existe), `ChangeDetector` (emite só quando o estado muda) e `POLL_INTERVAL = 50 ms`.
- `i18n`: `Lang::from_locale_tag("pt-BR") → Pt`, qualquer outro/indetectável → `En`; `Strings` com tooltip e rótulos do menu. `LockState::tooltip` passa a receber `Lang`.
- `theme`: `Theme::from_light_flag(Option<u32>)` — `Some(1)` → `Light`, `Some(0)` → `Dark`, `None` → `Dark`.
- `icon`: `render(state, theme, size_px) -> Vec<u8>` (RGBA). Glifos "A" e "1" desenhados a partir de um bitmap font 5×7 embutido, escalado para o tamanho pedido; aceso = cor de primeiro plano opaca, apagado = mesma cor com ~30% de opacidade; cor de primeiro plano branca no tema escuro e quase preta no claro. **Resolve a questão em aberto da spec:** sem arquivos `.ico`, sem build script; o tamanho vem de `SM_CXSMICON` (acompanha o DPI).
- `autostart`: trait `AutostartStore { read, write, delete }` + `Autostart::is_enabled(exe)` / `set_enabled(exe, bool)`. Habilitado = valor existe **e** aponta para o exe atual (comparação sem diferenciar maiúsculas, aspas normalizadas). Idempotente; erro do store é propagado e o estado exibido é sempre relido do store.

**Adaptador (`src/platform/`, `#[cfg(windows)]`, único lugar com `unsafe`)** — dependência única: `windows-sys`.
- `keyboard`: `GetKeyState(VK_CAPITAL) & 1` e `GetKeyState(VK_NUMLOCK) & 1`; constante `QUERIED_KEYS = [VK_CAPITAL, VK_NUMLOCK]`. Sem hook.
- `locale`: `GetUserDefaultUILanguage` + `LCIDToLocaleName` → tag BCP-47 → `Lang`.
- `theme`: lê `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize\SystemUsesLightTheme` (tema da barra de tarefas, não dos apps).
- `autostart_registry`: `AutostartStore` sobre `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`, valor `caps-widget` = `"<caminho do exe>"`. Caminho da chave parametrizável (para teste em subchave temporária).
- `instance`: mutex nomeado `Local\caps-widget-…` (namespace `Local` = por sessão); `ERROR_ALREADY_EXISTS` → sai com código 0.
- `tray`: janela **oculta de nível superior** (não `HWND_MESSAGE` — janelas só-mensagem não recebem broadcasts como `TaskbarCreated` e `WM_SETTINGCHANGE`). `Shell_NotifyIconW` com `NOTIFYICON_VERSION_4`; `SetTimer(POLL_INTERVAL)` → lê teclado → `ChangeDetector` → `NIM_MODIFY` (ícone + tooltip). `TaskbarCreated` → `NIM_ADD` de novo (R10 e bandeja indisponível no login). `WM_SETTINGCHANGE` com `"ImmersiveColorSet"` → relê tema e redesenha (R9). Clique direito → `TrackPopupMenu` (com `SetForegroundWindow` antes e `WM_NULL` depois — contorno documentado para o menu fechar corretamente).
- `main.rs`: `#![windows_subsystem = "windows"]` (R11), `SetProcessDpiAwarenessContext(PER_MONITOR_AWARE_V2)` para o ícone não ficar borrado, checa instância única, roda o loop `GetMessageW`. Em não-Windows, `main` só imprime que a plataforma não é suportada (mantém `cargo check` no devbox/Linux).

## Alternativas descartadas
| Alternativa | Por que não |
|---|---|
| Crate `tray-icon` (+ `muda`, event loop) | Muitas dependências transitivas e memória maior (ameaça R12); esconde o tratamento de `TaskbarCreated`/`WM_SETTINGCHANGE`, que precisamos controlar (R9, R10). |
| Crate `windows` (wrappers de alto nível) | Compilação bem mais lenta e API maior; `windows-sys` basta para ~20 chamadas. |
| Hook `WH_KEYBOARD_LL` | Proibido por R3 (privacidade). |
| Raw Input (`RIDEV_INPUTSINK`) | Também recebe todas as teclas — fere o espírito de R3. |
| Ícones `.ico` embutidos via `embed-resource`/build script | 8 arquivos × vários tamanhos para manter; render em código é testável (CA1, CA17) e se ajusta a qualquer DPI. |
| Polling de 250 ms | Não garante R2 (100 ms). |

## Mudanças
| Módulo / arquivo | Mudança | Requisitos |
|---|---|---|
| `src/lib.rs` | Vira raiz de módulos; `LockState` vai para `state` (reexportado) | — |
| `src/state.rs` | `ChangeDetector`, `POLL_INTERVAL` | R2 |
| `src/i18n.rs` | `Lang`, `Strings`, `tooltip(lang)` | R4, R5 |
| `src/theme.rs` | `Theme` | R9 |
| `src/icon.rs` | Render RGBA de 8 variantes | R1, R9 |
| `src/autostart.rs` | Trait + regras | R7 |
| `src/platform/{keyboard,locale,theme,autostart_registry,instance,tray}.rs` | Adaptadores Win32 | R1–R3, R5–R11 |
| `src/main.rs` | Subsystem windows, DPI, instância única, loop | R8, R11 |
| `Cargo.toml` | `windows-sys = "0.61"` com features mínimas (`target.'cfg(windows)'`) | — |
| `tests/*.rs` | Testes por assunto (abaixo) | todos |
| `docs/architecture.md`, `CHANGELOG.md`, `docs/specs/0001-…/manual-test.md` | Fluxos, entrada no Unreleased, roteiro manual | — |

## Contratos alterados
- API interna da lib: `LockState::tooltip()` → `LockState::tooltip(Lang)`. Sem consumidores externos (`publish = false`); os testes de `tests/lock_state.rs` são atualizados.
- Sistema: valor `caps-widget` em `HKCU\…\Run` — só escrito por ação do usuário. Nome do mutex `Local\caps-widget-…` é contrato entre versões (mudá-lo permitiria duas instâncias de versões diferentes).

## Estratégia de teste
| Requisito / CA | Tipo de teste | Onde |
|---|---|---|
| R1 / CA1 | unitário: mapeamento estado → variante; 4 renders distintos; pixels do glifo aceso mais opacos que apagado | `tests/icon.rs` |
| R2 / CA3, CA4 | unitário: `ChangeDetector`; `POLL_INTERVAL <= 50ms` | `tests/change_detector.rs` |
| R3 / CA6 | unitário (Windows): `QUERIED_KEYS == [VK_CAPITAL, VK_NUMLOCK]`; guarda: nenhum `.rs` em `src/` contém `SetWindowsHookEx` | `tests/keyboard_keys.rs`, `tests/no_keyboard_hook.rs` |
| R4 / CA7, CA8 | unitário: `pt-BR`, `pt-PT` → pt; `en-US`, `es-ES`, `""` → en; textos exatos | `tests/i18n.rs` |
| R5 / CA9 | unitário: rótulos por idioma | `tests/i18n.rs` |
| R7 / CA11–CA15 | unitário com `FakeStore` (inclui store que falha) | `tests/autostart.rs` |
| R7 (adaptador) | integração (Windows): round-trip em subchave temporária `HKCU\Software\caps-widget-test-<pid>`, apagada no fim | `tests/autostart_registry.rs` |
| R4/R9 (adaptadores) | integração (Windows): `locale` devolve tag não vazia; `theme` devolve um `Theme` sem pânico | `tests/platform_smoke.rs` |
| R9 / CA17 | unitário: 8 renders distintos; `None` → escuro | `tests/icon.rs` |
| R1, R2, R5, R6, R8–R12 / CA2, CA5, CA10, CA16, CA18–CA21 | manual | `docs/specs/0001-tray-lock-indicator/manual-test.md` (resultado colado no PR) |

## Riscos e mitigação
- **`GetKeyState` desatualizado em processo de fundo** (o estado de teclado é por thread de entrada). Mitigação: T5 valida cedo com `examples/poll_keys.rs` (imprime mudanças enquanto outra janela tem o foco). Plano B que respeita R3: `AttachThreadInput` à thread da janela em primeiro plano antes de ler, ou `GetKeyboardState` após `GetKeyState(0)`. Se nenhum servir, paro e discutimos.
- **Menu de contexto que não fecha** ao clicar fora: contorno `SetForegroundWindow` + `PostMessage(WM_NULL)`.
- **Ícone borrado em DPI alto**: DPI awareness por monitor + render no tamanho de `SM_CXSMICON`.
- **Antivírus**: sem hook e sem injeção; risco baixo.
- **`unsafe`**: restrito a `src/platform/`, cada bloco com `// SAFETY:`; revisado no `/review` pelo `security-reviewer`.

## Rollback
App ainda não distribuído: reverter o PR. Para quem testou com autostart ligado: desmarcar no menu antes, ou remover o valor `caps-widget` em `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.

## Decisões que viram ADR
- → ADR 0004: Win32 direto via `windows-sys` (sem `tray-icon`), ícones renderizados em código e detecção por polling sem hook de teclado.
