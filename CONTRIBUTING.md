# Contribuindo

Obrigado pelo interesse! Este projeto é desenvolvido com apoio de IA (Claude Code), mas contribuições manuais seguem exatamente o mesmo fluxo.

## Ambiente
1. Instale [devbox](https://www.jetify.com/devbox) e [direnv](https://direnv.net/) (opcional, recomendado).
2. `direnv allow` (ou `devbox shell`).
3. `just setup` — instala dependências e git hooks.
4. `just check` — deve passar antes de qualquer mudança.

Todos os comandos estão em `just --list`.

## Fluxo
1. Para mudanças não triviais, abra uma issue antes para alinhar o problema.
2. Crie uma branch a partir da `main`: `feat/…`, `fix/…`, `docs/…`, `refactor/…`, `chore/…`.
3. Mudança de comportamento vem com teste; correção de bug vem com teste que a reproduz.
4. Commits em [Conventional Commits](https://www.conventionalcommits.org/pt-br/v1.0.0/), em inglês.
5. Abra o PR preenchendo o template. O título do PR também deve ser Conventional — ele vira o commit do squash.

Features maiores usam specs em `docs/specs/` (veja [docs/workflow.md](docs/workflow.md)). Decisões arquiteturais ficam em [docs/adr/](docs/adr/).

## Contribuições geradas com IA
São bem-vindas, com as mesmas exigências: você entende e se responsabiliza pelo código, os testes provam o comportamento e o PR descreve como foi verificado.

## Código de conduta
Ao participar você concorda com o [Código de Conduta](CODE_OF_CONDUCT.md).
