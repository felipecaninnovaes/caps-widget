---
paths:
  - "src/**"
  - "tests/**"
  - "Cargo.toml"
---
- Núcleo (`src/lib.rs` e módulos de domínio) é puro e testável em qualquer SO: sem `cfg(windows)`, sem chamadas ao sistema.
- Todo acesso a Win32 (estado de teclas, bandeja, mensagens de janela) fica em `src/platform/`, atrás de `#[cfg(windows)]`; `unsafe` só lá, com comentário `// SAFETY:` explicando a invariante.
- Sem `unwrap()`/`expect()` em código de produção fora de `main` — propague o erro.
- Testes de integração em `tests/<assunto>.rs`; um único teste: `just test --test <assunto>` ou `just test <nome_do_teste>`.
