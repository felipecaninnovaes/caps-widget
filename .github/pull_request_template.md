## Resumo
<!-- O que muda e por quê, em 1–3 frases. -->

**Spec:** <!-- docs/specs/NNNN-slug ou "n/a" -->
**Issue:** <!-- Closes #123 -->

## Como foi verificado
<!-- Comandos executados e resultado (cole o final da saída). -->
```
just check
```

## Riscos e rollback
<!-- O que pode quebrar e como desfazer. -->

## Checklist
- [ ] Título do PR em Conventional Commits (vira o commit do squash)
- [ ] `just check` passa localmente
- [ ] Testes cobrem os critérios de aceite / reproduzem o bug corrigido
- [ ] `CHANGELOG.md` atualizado (se visível ao usuário)
- [ ] ADR criado/atualizado (se houve decisão difícil de reverter)
- [ ] Sem segredos, dados pessoais ou credenciais no diff
- [ ] Dependências novas justificadas (licença, manutenção)
