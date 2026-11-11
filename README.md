# caps-widget

Indicador de **Caps Lock** e **Num Lock** na bandeja do Windows — para quem usa teclado sem LED (ou com LED escondido) e quer ver o estado das teclas de relance.

> Estado: esqueleto inicial. O núcleo de estado existe e é testado; leitura via Win32 e ícone na bandeja vêm na primeira feature.

**Não é** remapeador de teclas, keylogger nem app multiplataforma. O widget só lê o estado das teclas de bloqueio — nunca captura o que é digitado.

## Requisitos
- Windows 10/11
- [rustup](https://rustup.rs/) — a versão do Rust é fixada em `rust-toolchain.toml` e instalada automaticamente
- [just](https://github.com/casey/just), [pre-commit](https://pre-commit.com/), [gitleaks](https://github.com/gitleaks/gitleaks):
  ```powershell
  winget install Rustlang.Rustup Casey.Just Gitleaks.Gitleaks GitHub.cli
  python -m pip install --user pre-commit
  ```

## Rodando
```powershell
just setup        # hooks do git + dependências
cargo run         # executa o widget
just check        # fmt-check + lint + testes
just test --test lock_state   # um arquivo de teste
just build        # binário de release em target/release/caps-widget.exe
```

## Contribuindo
- Branches curtas a partir de `main` (`feat/`, `fix/`, ...), PR com squash merge.
- Commits e título do PR em [Conventional Commits](https://www.conventionalcommits.org/) (inglês); docs em pt-BR.
- Todo comportamento novo tem teste; `just check` precisa passar.
- Detalhes em [CONTRIBUTING.md](CONTRIBUTING.md). Vulnerabilidades: [SECURITY.md](SECURITY.md).

Arquitetura: [docs/architecture.md](docs/architecture.md) · Decisões: [docs/adr/](docs/adr/).

## Desenvolvimento com IA
O repositório é preparado para **Claude Code**: contexto em `CLAUDE.md`, fluxo spec-driven (`/spec → /plan → /implement → /review → /ship`), hooks que formatam arquivos, bloqueiam segredos e impedem encerrar com `just check` vermelho. Veja [docs/workflow.md](docs/workflow.md) e [docs/ai-guidelines.md](docs/ai-guidelines.md).

## Licença
[MIT](LICENSE)
