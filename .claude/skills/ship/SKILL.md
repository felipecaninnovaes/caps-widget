---
name: ship
description: Finaliza a branch atual — verifica, organiza commits Conventional, faz push e abre o PR com gh. Só roda quando o usuário invoca /ship.
argument-hint: "[--draft]"
disable-model-invocation: true
---

# /ship — commit + PR

Opções: $ARGUMENTS

1. **Pré-condições** (pare e relate se alguma falhar):
   - não está em `main`/`master`;
   - `just check` passa;
   - `just scan-secrets` limpo (se o gitleaks estiver disponível);
   - `git status` sem arquivos inesperados — mostre-o ao usuário.
2. **Commits:** o que estiver pendente vira commits Conventional coerentes (inglês, imperativo, ≤ 72 caracteres no assunto). Nunca `--no-verify`; se um hook falhar, corrija a causa.
3. **CHANGELOG:** garanta a entrada em `[Unreleased]` se houver mudança visível ao usuário.
4. **Push:** `git push -u origin <branch>` (pedirá confirmação).
5. **PR:** `gh pr create` com
   - título Conventional — vira a mensagem do squash merge;
   - corpo seguindo [.github/pull_request_template.md](../../../.github/pull_request_template.md): resumo, link da spec, como foi verificado (comandos e resultado), riscos, checklist preenchido;
   - `--draft` se o usuário pediu ou se há pendências.
6. Devolva o link do PR. **Não faça merge**: isso é decisão humana.
