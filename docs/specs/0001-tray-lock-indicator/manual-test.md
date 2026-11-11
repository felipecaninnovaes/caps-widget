# 0001 — Roteiro de teste manual

Executar no Windows 10/11 com o build de release (`just build` → `target\release\caps-widget.exe`). Cole a tabela preenchida no PR.

- **Executado por:**
- **Data:**
- **Windows (versão/build):**
- **Escala de exibição:**

## Pré-condições
- Nenhuma instância rodando (`Get-Process caps-widget` vazio).
- Autostart desligado: `reg query HKCU\Software\Microsoft\Windows\CurrentVersion\Run /v caps-widget` → valor não encontrado.

## Roteiro
| # | CA | Passos | Esperado | Resultado |
|---|---|---|---|---|
| 0 | risco T5 | `cargo run --example poll_keys -- 30`; com **outra janela em foco**, alternar Caps e Num algumas vezes | Uma linha por troca, com o estado correto | |
| 1 | CA20 | Ligar Caps; abrir o `.exe` por duplo clique | Nenhuma janela nem console aparece | |
| 2 | CA2 | (continuação) Olhar a bandeja (expandir `^` se preciso) | Um ícone com "A" aceso e "1" conforme Num | |
| 3 | CA5 | Com outra janela em foco, apertar Caps e Num várias vezes | Ícone e tooltip mudam sem atraso perceptível; tooltip no idioma do Windows | |
| 4 | CA16 | Abrir o `.exe` de novo | Continua um único ícone; `Get-Process caps-widget` mostra 1 processo | |
| 5 | CA19 | `Stop-Process -Name explorer` (o Explorer reabre sozinho; senão, `explorer.exe`) | Ícone reaparece com o estado atual | |
| 6 | CA18 | Configurações → Personalização → Cores → modo do Windows claro/escuro | Ícone troca para a variante legível sem reiniciar o app | |
| 6b | R9 (DPI) | Configurações → Sistema → Tela → Escala: 100% → 150% (sem sair da sessão) | Ícone continua nítido (redesenhado no novo tamanho), não borrado | |
| 7 | R5 | Clique direito no ícone; depois clicar fora | Menu com "Iniciar com o Windows" (desmarcado) e "Sair" no idioma do Windows; fecha ao clicar fora | |
| 8 | R7 | Marcar "Iniciar com o Windows"; rodar o `reg query` acima; reabrir o menu | Valor `caps-widget` = `"<caminho do exe>"`; item marcado | |
| 9 | R7 | Desmarcar; `reg query` de novo | Valor removido; item desmarcado | |
| 10 | CA21 | Deixar ocioso 5 min; Gerenciador de Tarefas → Detalhes | Memória (conjunto de trabalho privado) < 10 MB; CPU ~0% | |
| 11 | CA10 | Clique direito → Sair | Ícone some; processo termina | |

## Observações
<!-- Desvios, prints, valores medidos -->

### Execução automatizada parcial (agente, 2026-11-11, Windows 11 Pro 10.0.26300)
Itens observáveis sem olhar a tela, executados via PowerShell. Itens visuais (2, 3, 5–9, 11) continuam para execução humana.

| # | CA | Como | Resultado |
|---|---|---|---|
| 0 | gate T5 | `poll_keys` release, oculto, em segundo plano; `WScript.Shell.SendKeys` Caps, Num, Num, Caps (900 ms entre teclas) | ✅ 5 linhas: estado inicial + uma por troca, todas corretas. Numa 1ª rodada a **leitura inicial veio obsoleta** (Caps ligado) e se corrigiu sozinha antes da 1ª tecla; demais leituras corretas. |
| 1 | CA20 (parcial) | `Start-Process` do `.exe` release | ✅ processo vivo, `MainWindowHandle = 0` (sem janela visível); sem console (subsystem `windows`). Duplo clique não testado. |
| 4 | CA16 | segunda execução com a primeira rodando | ✅ segunda saiu com código 0; `Get-Process caps-widget` = 1 |
| 10 | CA21 (parcial) | ocioso **30 s** (não 5 min) | ✅ CPU 0 ms no período; memória privada 1,38 MB; conjunto de trabalho 9,71 MB (inclui DLLs compartilhadas; o Gerenciador de Tarefas mostra o privado) |
