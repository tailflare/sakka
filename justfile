set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

[default]
[private]
default:
    just --list

# Core Pipeline
build target="std" *cargo_args:
    @just _run build {{ target }} {{ cargo_args }}

check target="std" *cargo_args:
    @just _run check {{ target }} {{ cargo_args }}

test target="std" *cargo_args:
    @just _run test {{ target }} {{ cargo_args }}

sanitize:
    @just tidy
    @just sanity

sanity:
    @just validate
    @just build std
    @just build no-std
    @just test std -- --quiet
    @just test no-std -- --quiet

# Release
release version:
    cargo +stable set-version {{ version }}
    @just sanity
    @echo "Performing dry run..."
    cargo +stable publish --dry-run --allow-dirty
    git commit -am "chore: release {{ version }}"
    git tag -a "v{{ version }}" -m "Release {{ version }}"
    git push origin main
    git push origin "v{{ version }}"
    @echo "Release {{ version }} tagged. CI will now publish the crate and create the GitHub release."

# Housekeeping
fmt:
    cargo +nightly fmt --all

fmt-check:
    cargo +nightly fmt --all -- --check

clippy:
    cargo +stable clippy --all-features --fix --allow-dirty

lint:
    cargo +stable clippy --all-features -- -D warnings

validate:
    @just fmt-check
    @just lint

tidy:
    @just fmt
    @just clippy

ci_sanity:
    @just sanity

ci_crate_publish:
    cargo +stable publish --workspace

# Private helpers
[private]
_run command target *cargo_args:
    @just _run-{{ target }} {{ command }} --workspace {{ cargo_args }}

[private]
_run-std command *cargo_args:
    cargo +stable {{ command }} {{ cargo_args }}

[private]
_run-no-std command *cargo_args:
    {{ if command == "test" { "cargo +stable test --no-default-features" } else { "cargo +stable " + command + " --no-default-features --target thumbv7m-none-eabi" } }} {{ cargo_args }}
