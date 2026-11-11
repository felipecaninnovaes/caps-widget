---
paths:
  - ".github/workflows/**"
---
- CI chama apenas receitas do `justfile` via `devbox run -- just <receita>`; não duplique comandos da stack no YAML.
- `permissions:` mínimas no nível do workflow (`contents: read`) e ampliadas só no job que precisa.
- Nunca use `pull_request_target` com checkout do código do PR. Nunca interpole `${{ github.event.* }}` direto em `run:` — passe por `env:`.
- Actions de terceiros: só de publicadores conhecidos; o Dependabot mantém as versões atualizadas.
- Nomes de jobs `check` e `pr-title` são usados no ruleset da `main`; renomear exige atualizar o ruleset.
