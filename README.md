# repositorio-modelo

Modelo de repositório **agnóstico de linguagem** para projetos desenvolvidos com **Claude Code**: contexto enxuto para o agente, fluxo spec-driven, regras críticas garantidas por hooks e CI, e um contrato único de comandos via `just`.

<!-- TODO(bootstrap): este README descreve o modelo; o /bootstrap o reescreve para o projeto. -->

## Começando um projeto novo

```bash
# 1. Crie o repositório a partir do modelo (ou marque este repo como "Template repository" no GitHub)
gh repo create meu-projeto --template <owner>/repositorio-modelo --private --clone
cd meu-projeto

# 2. Ambiente (Linux/macOS/WSL)
direnv allow            # ou: devbox shell

# 3. Inicialize com o agente
claude
> /bootstrap
```

O `/bootstrap` entrevista você (stack, comandos, arquitetura, distribuição), implementa as receitas do `justfile`, fixa as ferramentas no `devbox.json`, ajusta CI/Dependabot/licença, preenche todos os `TODO(bootstrap)` e só termina com `just check` verde.

### Windows sem devbox
Os hooks rodam no Git Bash. Instale as ferramentas uma vez:
```powershell
winget install Casey.Just jqlang.jq Gitleaks.Gitleaks GitHub.cli
pipx install pre-commit   # ou: pip install --user pre-commit
```
Sem `jq`, os hooks usam `node` ou `python` como fallback.

## Fluxo de trabalho

| Comando | Faz | Saída |
|---|---|---|
| `/spec <ideia>` | entrevista até haver critérios de aceite testáveis | `docs/specs/NNNN-slug/spec.md` |
| `/plan NNNN` | explora o código, escolhe abordagem, quebra em tarefas | `plan.md`, `tasks.md` |
| `/implement NNNN` | teste → código → `just check` → commit, por tarefa | commits na branch `feat/NNNN-slug` |
| `/review` | `spec-reviewer` + `security-reviewer` em paralelo | achados por severidade |
| `/ship` | verifica, push e `gh pr create` | link do PR |
| `/adr <decisão>` | registra decisão difícil de reverter | `docs/adr/NNNN-slug.md` |

Detalhes, tamanhos de mudança e gates: [docs/workflow.md](docs/workflow.md).

## Estrutura

```
CLAUDE.md                 contexto sempre carregado (curto, só o não-óbvio)
justfile                  contrato: setup check fmt fmt-check fmt-file lint test build
devbox.json / .envrc      ferramentas fixadas + ativação via direnv
.claude/
  settings.json           permissões (allow/ask/deny) e hooks
  hooks/                  guard-secrets, guard-git, format-file, stop-check, session-start
  skills/                 bootstrap, spec, plan, implement, review, adr, ship
  agents/                 spec-reviewer, security-reviewer, test-writer
  rules/                  regras carregadas só ao tocar certos caminhos
docs/
  workflow.md             fluxo spec-driven
  architecture.md         fronteiras e integrações (o que o código não mostra)
  adr/                    decisões arquiteturais
  specs/                  specs, planos e tarefas por mudança
  ai-guidelines.md        princípios e fontes (por que o modelo é assim)
.github/                  CI (devbox + just), título de PR, Dependabot, templates
```

## O que é garantido (não depende do modelo obedecer)

- Leitura de `.env`, chaves e credenciais locais é negada.
- Gravação de segredos ou escrita em `.env` é bloqueada; gitleaks no pre-commit e no CI.
- Sem commit/push direto na `main`, sem force push, sem `--no-verify`.
- Push, PR, merge e release pedem confirmação.
- Arquivo editado é formatado; o turno não termina com `just check` vermelho.
- Conventional Commits no commit e no título do PR.

Princípios e fontes: [docs/ai-guidelines.md](docs/ai-guidelines.md). Decisões da fundação: [ADR 0002](docs/adr/0002-ai-development-foundation.md).

## Personalização
- Preferências pessoais: `CLAUDE.local.md` e `.claude/settings.local.json` (ignorados pelo git).
- Novo procedimento recorrente → nova skill em `.claude/skills/<nome>/SKILL.md`.
- Regra que a IA ignora repetidamente → transforme em hook.

## Licença
[MIT](LICENSE)
