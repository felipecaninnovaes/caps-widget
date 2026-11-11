# 0004 — Win32 direto via `windows-sys`, ícones em código e polling sem hook

- **Status:** aceito
- **Data:** 2026-11-11
- **Decisores:** Felipe CN

## Contexto
A spec [0001](../specs/0001-tray-lock-indicator/spec.md) exige refletir Caps/Num Lock em até 100 ms (R2) sem hook global de teclado (R3), reagir à troca de tema (R9) e à reinicialização do Explorer (R10), com consumo ocioso < 10 MB (R12). O ADR 0003 deixou a escolha das crates de plataforma para esta spec.

## Opções consideradas
1. **Crate `tray-icon` (+ `muda`, event loop)** — API pronta. Contras: muitas dependências transitivas, mais memória, e esconde `TaskbarCreated`/`WM_SETTINGCHANGE`, que precisamos tratar.
2. **Crate `windows`** — wrappers seguros. Contras: compilação lenta, API grande para ~20 chamadas.
3. **`windows-sys` direto** — bindings crus, sem dependências transitivas além das do próprio projeto `windows-rs`. Contras: `unsafe` em todo acesso.
- Detecção: **hook `WH_KEYBOARD_LL`** e **Raw Input** recebem todas as teclas (ferem R3); **polling de `GetKeyState` a cada 50 ms** consulta só as duas teclas.
- Ícones: **`.ico` embutidos** (8 variantes × vários tamanhos, build script) vs **render RGBA em código** no tamanho pedido pelo sistema.

## Decisão
Escolhemos **`windows-sys` (MIT OR Apache-2.0), polling de `GetKeyState(VK_CAPITAL | VK_NUMLOCK)` a cada 50 ms e ícones renderizados em código**, porque é a combinação que cumpre R3 por construção, mantém o binário pequeno, deixa o tratamento de mensagens do shell sob nosso controle e torna as 8 variantes de ícone testáveis no núcleo puro.

## Consequências
- ✅ O processo nunca vê o que é digitado; menos alertas de antivírus.
- ✅ Uma dependência, sem build script; ícone nítido em qualquer DPI.
- ⚠️ `unsafe` concentrado em `src/platform/`, cada bloco com `// SAFETY:`.
- ⚠️ Polling acorda o processo 20×/s (custo de CPU desprezível, mas não zero).
- ⚠️ Se `GetKeyState` não refletir mudanças com o processo em segundo plano, o plano B (`AttachThreadInput`/`GetKeyboardState`) fica restrito a `platform::keyboard`.
- ⚠️ Glifos em bitmap 5×7 são simples; um desenho mais elaborado exigirá trocar o render.
