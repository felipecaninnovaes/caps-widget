# 0001 — Registrar decisões arquiteturais com ADRs

- **Status:** aceito
- **Data:** 2026-11-11

## Contexto
Agentes de IA começam cada sessão sem memória de reuniões, conversas ou motivos de escolhas passadas, e tendem a re-sugerir alternativas já descartadas. Pessoas novas no projeto têm o mesmo problema.

## Opções consideradas
1. **Sem registro formal** — decisões ficam em PRs e conversas. Barato, mas inencontrável.
2. **ADRs em Markdown no repositório (MADR curto)** — versionados com o código, legíveis por humanos e por agentes.
3. **Wiki externa** — fora do alcance do agente e dessincroniza do código.

## Decisão
Opção 2. Decisões difíceis de reverter são registradas em `docs/adr/NNNN-slug.md` via `/adr`. ADR aceito é imutável; mudança de ideia gera um ADR novo que o substitui.

## Consequências
- ✅ O agente encontra o "porquê" sem perguntar; alternativas rejeitadas ficam registradas.
- ⚠️ Exige disciplina: o `/plan` marca decisões "→ ADR" para não esquecer.
