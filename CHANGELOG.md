# Changelog

Todas as mudanças relevantes para usuários deste projeto. Formato baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/); versionamento [SemVer](https://semver.org/lang/pt-BR/).

## [Unreleased]

### Added
- Esqueleto do projeto em Rust: modelo `LockState` com tooltip de Caps/Num Lock.
- Ícone na bandeja do Windows que mostra Caps Lock ("A") e Num Lock ("1") acesos ou apagados, atualizado em até 100 ms, sem hook de teclado.
- Tooltip e menu em português (Windows em `pt-*`) ou inglês.
- Menu com "Iniciar com o Windows" (desligado por padrão, por usuário, sem administrador) e "Sair".
- Variantes do ícone para barra de tarefas clara e escura, trocadas ao vivo; o ícone volta sozinho se o Explorer reiniciar.
- Uma instância por sessão; roda sem janela nem console.

<!-- Seções possíveis: Added, Changed, Deprecated, Removed, Fixed, Security -->
