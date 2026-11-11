# 0002 — Fundação para desenvolvimento assistido por IA

- **Status:** aceito
- **Data:** 2026-11-11

## Contexto
Todo projeto começa a partir deste modelo e é desenvolvido majoritariamente com Claude Code. A fundação precisa ser agnóstica de linguagem, funcionar em Windows (Git Bash/PowerShell) e Linux (devbox + direnv), e seguir as práticas atuais de desenvolvimento com agentes (ver [ai-guidelines.md](../ai-guidelines.md)).

## Opções consideradas
- **Ferramenta de IA:** só Claude Code · AGENTS.md multi-ferramenta. → só Claude Code (aproveita skills, subagentes, hooks e permissões nativos).
- **Fluxo:** spec-driven · plano leve · TDD-first. → spec-driven com tamanho proporcional à mudança.
- **Contrato de comandos:** `just` · Makefile · scripts sh+ps1 · Taskfile. → `just` (cross-platform, receitas com argumentos, sem as armadilhas do make no Windows).
- **Ambiente:** devbox + direnv · devcontainer · manual. → devbox + direnv (versões fixadas via Nix; CI usa o mesmo ambiente).
- **Hooks:** bash + jq · Python · Node. → bash + jq, com fallback para node/python quando o jq não está instalado.
- **Git:** trunk + PR curto · GitFlow · direto na main. → trunk + PR curto, Conventional Commits, squash.
- **Idioma:** docs pt-BR, código/commits em inglês.
- **Público:** open source, MIT.

## Decisão
1. `CLAUDE.md` curto contendo só o que o agente não descobre lendo o código; detalhes em `docs/` e skills (divulgação progressiva).
2. Regras sem exceção são **determinísticas**: deny rules (segredos), hooks (`guard-secrets`, `guard-git`, `format-file`, `stop-check`), pre-commit e CI. Preferências ficam como orientação.
3. O `justfile` é o contrato único (`setup check fmt fmt-check fmt-file lint test build`) chamado por agente, hooks, pre-commit e CI; o `/bootstrap` implementa as receitas por stack.
4. Ciclo `/spec → /plan → /implement → /review → /ship`, com artefatos em `docs/specs/` e revisão por subagentes independentes.
5. CI no GitHub Actions rodando dentro do devbox; gitleaks no pre-commit e no CI; Dependabot; título de PR validado.

## Consequências
- ✅ Mesmo comportamento em qualquer linguagem; trocar de stack só muda as receitas do `justfile`.
- ✅ O agente tem um check pass/fail e não consegue encerrar com ele vermelho.
- ⚠️ No Windows sem devbox é preciso instalar `just`, `jq`, `gitleaks` e `pre-commit` manualmente.
- ⚠️ O hook `Stop` adiciona o tempo do `just check` ao fim dos turnos com código alterado (mitigado por cache do estado do working tree).
