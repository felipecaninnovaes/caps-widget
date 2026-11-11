# Contrato de comandos do projeto.
# Claude Code, hooks, pre-commit e CI chamam SOMENTE estas receitas:
# troque a implementação, nunca o nome. Receitas ainda não configuradas
# estão marcadas com TODO(bootstrap) e terminam com sucesso.

set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

# Lista as receitas
default:
    @just --list --unsorted

# Instala dependências da stack e os git hooks
setup:
    pre-commit install --install-hooks --hook-type pre-commit --hook-type commit-msg
    @echo "setup: TODO(bootstrap) instalar dependencias da stack"

# Verificação completa (exigida pelo hook Stop e pelo CI)
check: fmt-check lint test

# Formata todo o código
fmt:
    @echo "fmt: TODO(bootstrap)"

# Falha se algum arquivo não estiver formatado
fmt-check:
    @echo "fmt-check: TODO(bootstrap)"

# Formata um único arquivo (chamado pelo hook PostToolUse a cada edição)
fmt-file file:
    @echo "fmt-file: TODO(bootstrap) {{file}}"

# Lint + typecheck
lint:
    @echo "lint: TODO(bootstrap)"

# Roda os testes; aceita caminho/filtro: just test path/to/test
test *args:
    @echo "test: TODO(bootstrap) {{args}}"

# Build de produção
build:
    @echo "build: TODO(bootstrap)"

# Procura segredos no histórico git
scan-secrets:
    gitleaks git --redact --no-banner .

# Roda todos os hooks do pre-commit em todos os arquivos
hooks-run:
    pre-commit run --all-files --show-diff-on-failure

# Atualiza as versões dos hooks do pre-commit
hooks-update:
    pre-commit autoupdate
