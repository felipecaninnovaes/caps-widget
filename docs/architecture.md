# Arquitetura

> Só o que **não** é óbvio lendo o código: fronteiras, fluxos, integrações e o porquê. Estrutura de pastas o agente descobre sozinho — não liste aqui.

## Contexto
Usuário de Windows sem LED de Caps/Num Lock. O widget roda em segundo plano, lê o estado das teclas de bloqueio no próprio SO e o mostra como ícone na bandeja. Não há rede, servidor nem armazenamento de dados do usuário.

## Componentes
| Componente | Responsabilidade | Depende de |
|---|---|---|
| Núcleo (`src/lib.rs`) | `LockState` e regras de apresentação (tooltip, qual ícone mostrar). Puro, testável em qualquer SO. | — |
| Adaptador de teclado (`src/platform/`, previsto) | Lê Caps/Num Lock via Win32 (`GetKeyState`) e detecta mudanças. | Win32 (`windows-sys`, a decidir na spec) |
| Bandeja (`src/platform/`, previsto) | Cria/atualiza o ícone e tooltip; menu "Sair". | Win32 / `tray-icon` (a decidir na spec) |
| `main` | Liga adaptadores ao núcleo e roda o loop de mensagens. | todos |

Regra de fronteira: o núcleo não conhece Win32; só `src/platform/` usa `unsafe`/`cfg(windows)`.

## Fluxos principais
1. Usuário pressiona Caps Lock → adaptador de teclado detecta a mudança (hook de teclado de baixo nível ou polling — a decidir).
2. Adaptador produz um novo `LockState` → núcleo decide ícone e tooltip.
3. Bandeja atualiza o ícone.

## Integrações externas
| Sistema | Para quê | Como autenticamos | Variáveis de ambiente |
|---|---|---|---|
| API Win32 | Estado das teclas e ícone na bandeja | — (local) | — |

## Decisões
Ver [adr/](adr/) — em especial [0003](adr/0003-rust-windows-tray-stack.md) (stack).
