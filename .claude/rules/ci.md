---
paths:
  - ".github/workflows/**"
---
- CI chama apenas receitas do `justfile`; não duplique comandos da stack no YAML.
- Job `hygiene` (Linux) roda via `devbox run -- just <receita>`. Job `check` roda em `windows-latest` **sem devbox** (o app é só para Windows e o devbox não roda lá): chama `just <receita>` direto, com a toolchain fixada em `rust-toolchain.toml`.
- `permissions:` mínimas no nível do workflow (`contents: read`) e ampliadas só no job que precisa.
- Nunca use `pull_request_target` com checkout do código do PR. Nunca interpole `${{ github.event.* }}` direto em `run:` — passe por `env:`.
- Actions de terceiros: só de publicadores conhecidos; o Dependabot mantém as versões atualizadas.
- Nomes de jobs `check`, `hygiene` e `pr-title` são usados no ruleset da `main`; renomear exige atualizar o ruleset.
