# Contrato de comandos do projeto.
# Claude Code, hooks, pre-commit e CI chamam SOMENTE estas receitas:
# troque a implementação, nunca o nome.

set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

# Lista as receitas
default:
    @just --list --unsorted

# Instala dependências da stack e os git hooks
setup:
    pre-commit install --install-hooks --hook-type pre-commit --hook-type commit-msg
    cargo fetch

# Verificação completa (exigida pelo hook Stop e pelo CI)
check: fmt-check lint test

# Formata todo o código
fmt:
    cargo fmt --all

# Falha se algum arquivo não estiver formatado
fmt-check:
    cargo fmt --all --check

# Formata um único arquivo (chamado pelo hook PostToolUse a cada edição); ignora não-Rust
[windows]
fmt-file file:
    if ('{{file}}' -like '*.rs') { rustfmt '{{file}}' }

[unix]
fmt-file file:
    case '{{file}}' in *.rs) rustfmt '{{file}}' ;; esac

# Lint + typecheck
lint:
    cargo clippy --all-targets --locked -- -D warnings

# Roda os testes; aceita caminho/filtro: just test --test lock_state | just test tooltip
test *args:
    cargo test --locked {{args}}

# Build de produção
build:
    cargo build --release --locked

# Procura segredos no histórico git
scan-secrets:
    gitleaks git --redact --no-banner .

# Roda todos os hooks do pre-commit em todos os arquivos
hooks-run:
    pre-commit run --all-files --show-diff-on-failure

# Atualiza as versões dos hooks do pre-commit
hooks-update:
    pre-commit autoupdate
