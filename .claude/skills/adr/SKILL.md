---
name: adr
description: Registra uma decisão arquitetural (ADR) em docs/adr/ no formato MADR curto. Use quando uma decisão for difícil de reverter — stack, formato de dados, contrato público, infraestrutura, segurança, dependência estrutural — ou quando um plano marcar "→ ADR".
argument-hint: "[título da decisão]"
---

# /adr — registrar decisão

Decisão: $ARGUMENTS

1. **Número:** maior `docs/adr/NNNN-*.md` + 1 (4 dígitos). Slug em inglês, kebab-case.
2. **Colete:** contexto (forças em jogo), pelo menos 2 opções consideradas, decisão, consequências positivas e negativas. O que não estiver na conversa, no código ou na spec, pergunte — não invente motivação.
3. **Escreva** `docs/adr/NNNN-<slug>.md` a partir de [0000-template.md](../../../docs/adr/0000-template.md), com `Status: proposto`. Mude para `aceito` quando o usuário aprovar.
4. **Substituição:** se substitui outro ADR, marque o antigo como `substituído por NNNN` (é a única edição permitida num ADR aceito).
5. **Índice:** adicione a linha em [docs/adr/README.md](../../../docs/adr/README.md).
6. Se a decisão muda fronteiras do sistema, atualize `docs/architecture.md`.
