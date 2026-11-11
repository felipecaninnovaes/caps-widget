# Arquitetura

> Só o que **não** é óbvio lendo o código: fronteiras, fluxos, integrações e o porquê. Estrutura de pastas o agente descobre sozinho — não liste aqui.

## Contexto
Usuário de Windows sem LED de Caps/Num Lock. O widget roda em segundo plano, lê o estado das teclas de bloqueio no próprio SO e o mostra como ícone na bandeja. Não há rede, servidor nem armazenamento de dados do usuário.

## Componentes
| Componente | Responsabilidade | Depende de |
|---|---|---|
| Núcleo (`state`, `i18n`, `theme`, `icon`, `autostart`) | `LockState` e `ChangeDetector`; textos pt/en; tema → cor; render RGBA do ícone; regras de "Iniciar com o Windows" sobre o trait `AutostartStore`. Puro, testável em qualquer SO. | — |
| Adaptadores (`src/platform/`) | `keyboard` (`GetKeyState` só de Caps/Num), `locale`, `theme` e `autostart_registry` (registro `HKCU`), `instance` (mutex), `dpi`. | Win32 (`windows-sys`) |
| Bandeja (`platform::tray`) | Janela oculta de nível superior: timer de polling, ícone na bandeja, menu, reação a `TaskbarCreated` e `WM_SETTINGCHANGE`. | adaptadores + núcleo |
| `main` | DPI, instância única, chama `tray::run`. Subsystem `windows` (sem console). | todos |

Regra de fronteira: o núcleo não conhece Win32; só `src/platform/` usa `unsafe`/`cfg(windows)`.

## Fluxos principais
1. **Estado das teclas:** `WM_TIMER` a cada 50 ms → `keyboard::read()` → `ChangeDetector` → se mudou, `icon::render` + tooltip → `Shell_NotifyIconW(NIM_MODIFY)`. Sem hook de teclado: o processo nunca vê o que é digitado.
2. **Explorer reiniciado / login lento:** broadcast `TaskbarCreated` → `NIM_ADD` de novo com o estado atual. Falha no `NIM_ADD` inicial não encerra o app.
3. **Tema:** `WM_SETTINGCHANGE` → relê `SystemUsesLightTheme` → se mudou, redesenha.
4. **Menu:** clique direito (`WM_CONTEXTMENU` via `NOTIFYICON_VERSION_4`) → menu montado relendo o registro (a marca sempre reflete o estado real) → alterna o valor em `HKCU\…\Run` ou destrói a janela (Sair).

## Integrações externas
| Sistema | Para quê | Como autenticamos | Variáveis de ambiente |
|---|---|---|---|
| API Win32 | Estado das teclas, ícone na bandeja, idioma, tema | — (local) | — |
| Registro do usuário (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`, valor `caps-widget`) | Iniciar com o Windows; escrito só por ação do usuário | — (escopo do usuário) | — |

## Decisões
Ver [adr/](adr/) — em especial [0003](adr/0003-rust-windows-tray-stack.md) (stack) e [0004](adr/0004-win32-direct-polling-code-icons.md) (Win32 direto, polling sem hook, ícones em código).
