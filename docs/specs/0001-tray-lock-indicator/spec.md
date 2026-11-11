# 0001 — Indicador de Caps/Num Lock na bandeja

- **Status:** aprovada <!-- rascunho | aprovada | implementada | abandonada -->
- **Autor:** Felipe CN
- **Data:** 2026-11-11
- **ADRs relacionados:** [0003](../../adr/0003-rust-windows-tray-stack.md)

## Contexto e problema
Quem usa teclado sem LED de Caps Lock / Num Lock (notebooks, teclados compactos/sem fio) só descobre o estado das teclas depois de digitar errado — senhas recusadas, texto em maiúsculas, números que viram setas. Hoje o caps-widget é só um esqueleto: o modelo `LockState` existe, mas nada lê o teclado nem aparece na tela. Esta é a primeira versão utilizável.

## Objetivos
- Mostrar continuamente, na bandeja do Windows, o estado de Caps Lock e Num Lock num único ícone.
- Refletir a mudança praticamente na hora em que a tecla é pressionada.
- Ser leve e discreto o bastante para ficar sempre ligado, inclusive desde o login.

## Fora de escopo
- Linux, macOS ou qualquer plataforma além de Windows 10/11.
- Scroll Lock, Insert ou outras teclas.
- Overlay/OSD na tela, notificações, sons.
- Hook global de teclado (ver R3).
- Janela de configurações, cores/ícones personalizáveis, idiomas além de pt-BR e inglês.
- Ação no clique esquerdo do ícone (por ora não faz nada).
- Instalador, assinatura de código, atualização automática, distribuição/release.

## Requisitos
| ID | Requisito | Prioridade |
|---|---|---|
| R1 | Ao iniciar, o app mostra **um** ícone na bandeja cuja variante corresponde ao estado atual de Caps Lock e Num Lock: 4 combinações visualmente distinguíveis — letra "A" para Caps e dígito "1" para Num; aceso = ligado, apagado/esmaecido = desligado. | obrigatório |
| R2 | Quando Caps Lock ou Num Lock mudam (por qualquer teclado ou origem), ícone e tooltip passam a refletir o novo estado em até **100 ms**. | obrigatório |
| R3 | A detecção **não** instala hook global de teclado nem consulta outras teclas além de Caps Lock e Num Lock. | obrigatório |
| R4 | O tooltip descreve os dois estados por extenso, no idioma da interface do Windows: pt-BR (`Caps Lock: ligado \| Num Lock: desligado`) se o idioma for português (`pt-*`); inglês (`Caps Lock: on \| Num Lock: off`) em qualquer outro caso. | obrigatório |
| R5 | Clique direito no ícone abre menu com "Iniciar com o Windows" (marcável) e "Sair", no idioma de R4 (inglês: "Start with Windows", "Exit"). | obrigatório |
| R6 | "Sair" remove o ícone da bandeja e encerra o processo. | obrigatório |
| R7 | "Iniciar com o Windows" vem **desligado** por padrão. Marcar registra o executável atual para iniciar no login do usuário (escopo do usuário, sem administrador); desmarcar remove o registro. A marca no menu reflete o estado real do registro. | obrigatório |
| R8 | Só uma instância roda por sessão de usuário: iniciar o app com outra já rodando encerra a nova em silêncio, sem segundo ícone. | obrigatório |
| R9 | O ícone tem variantes legíveis em barra de tarefas com tema claro e escuro e troca de variante quando o tema do Windows muda, sem reiniciar o app. | obrigatório |
| R10 | Se o Explorer reiniciar (bandeja recriada), o ícone reaparece automaticamente com o estado atual. | obrigatório |
| R11 | O app roda sem janela visível e sem console. | obrigatório |
| R12 | Consumo ocioso: memória de trabalho < 10 MB e CPU média < 0,1%. | desejável |

## Critérios de aceite
Cada critério vira pelo menos um teste. Os marcados *(manual)* dependem do shell do Windows e são verificados por roteiro manual registrado no PR; a lógica por trás deles tem teste automatizado.

- **CA1 (R1)** — Dado Caps ligado e Num desligado, quando o estado é mapeado para ícone, então a variante é "A aceso, 1 apagado"; e as 4 combinações de estado produzem 4 variantes distintas.
- **CA2 (R1)** *(manual)* — Dado o app fechado e Caps ligado, quando o app inicia, então aparece um ícone na bandeja com "A" aceso.
- **CA3 (R2)** — Dada uma sequência de leituras em que Caps passa de desligado para ligado, quando o detector processa a nova leitura, então emite exatamente uma mudança; leituras repetidas iguais não emitem nada.
- **CA4 (R2)** — O intervalo de verificação do estado é ≤ 50 ms (meta de 100 ms com folga).
- **CA5 (R2)** *(manual)* — Dado o app rodando, quando o usuário aperta Caps Lock, então o ícone muda sem atraso perceptível.
- **CA6 (R3)** — O adaptador de teclado consulta apenas as teclas Caps Lock e Num Lock (teste sobre o conjunto de teclas consultadas) e não usa `SetWindowsHookEx` (revisão + busca no código no `/review`).
- **CA7 (R4)** — Dado idioma `pt-BR` ou `pt-PT`, e Caps ligado/Num desligado, então o tooltip é `Caps Lock: ligado | Num Lock: desligado`.
- **CA8 (R4)** — Dado idioma `en-US`, `es-ES` ou indetectável, e o mesmo estado, então o tooltip é `Caps Lock: on | Num Lock: off`.
- **CA9 (R5)** — Em português, os rótulos do menu são "Iniciar com o Windows" e "Sair"; nos demais idiomas, "Start with Windows" e "Exit".
- **CA10 (R6)** *(manual)* — Dado o app rodando, quando o usuário escolhe Sair, então o ícone some e o processo termina.
- **CA11 (R7)** — Dado nenhum registro de inicialização, quando o app consulta o autostart, então o estado é "desligado" e o item aparece desmarcado.
- **CA12 (R7)** — Dado autostart desligado, quando o usuário marca o item, então existe um registro de inicialização do usuário com o caminho do executável atual; ao desmarcar, o registro deixa de existir. (Testado contra armazenamento falso, sem tocar no registro real.)
- **CA13 (R7)** — Dado um registro de inicialização apontando para outro caminho (executável movido), quando o app consulta o estado, então o item aparece **desmarcado**; marcar sobrescreve com o caminho atual.
- **CA14 (R7)** — Marcar com autostart já ligado, ou desmarcar com ele já desligado, não altera nada nem falha (idempotência).
- **CA15 (R7)** — Dada falha ao gravar o registro, quando o usuário marca o item, então o item continua desmarcado e o app não encerra.
- **CA16 (R8)** *(manual)* — Dado o app rodando, quando o usuário o abre de novo, então continua existindo um único ícone e o segundo processo termina sem mensagem.
- **CA17 (R9)** — Para cada um dos 4 estados, tema claro e tema escuro produzem variantes distintas — 8 variantes no total; tema indetectável usa a variante de tema escuro.
- **CA18 (R9)** *(manual)* — Dado o app rodando, quando o usuário troca o tema do Windows, então o ícone muda para a variante do novo tema sem reiniciar.
- **CA19 (R10)** *(manual)* — Dado o app rodando, quando o Explorer é reiniciado, então o ícone reaparece com o estado atual.
- **CA20 (R11)** *(manual)* — Dado o executável de release, quando iniciado por duplo clique, então nenhuma janela ou console aparece.
- **CA21 (R12)** *(manual)* — Dado o app ocioso por 5 minutos, então o Gerenciador de Tarefas mostra memória < 10 MB e CPU ~0%.

## Casos de borda e erros
- **Bandeja indisponível ao iniciar** (login lento): não encerrar; adicionar o ícone quando a bandeja for criada (mesmo mecanismo de R10).
- **Falha ao gravar/remover autostart**: o menu mostra o estado real (CA15); sem crash.
- **Executável movido com autostart ligado**: CA13.
- **Idioma indetectável**: inglês. **Tema indetectável**: variante escura.
- **Repetição** de marcar/desmarcar autostart: idempotente (CA14).
- **Várias sessões** (outro usuário, Área de Trabalho Remota): uma instância por sessão.

## Invariantes (o que não pode mudar)
- O núcleo (`LockState` e regras de apresentação) continua puro: sem Win32, testável em qualquer SO (`.claude/rules/rust.md`).
- O app nunca lê, registra ou transmite teclas além do estado de Caps Lock e Num Lock; nenhum acesso à rede.
- Nenhuma escrita no sistema sem ação explícita do usuário (autostart só ao marcar o item).

## Restrições
- Windows 10 e 11, x64; sem privilégio de administrador.
- Privacidade: sem hook global de teclado (R3) — o processo não vê o que é digitado, e há menos alertas de antivírus.
- Performance: R12.
- Dependências novas (crates Win32/bandeja) justificadas no plano/PR, licença compatível com MIT.

## Questões em aberto
- [ ] Desenho final dos ícones (cores exatas; tamanhos 16/20/24/32 px para DPI alto) — decidir no `/plan`.

## Histórico de mudanças
| Data | Mudança | Motivo |
|---|---|---|
| 2026-11-11 | Criação | Primeira versão utilizável do app |
