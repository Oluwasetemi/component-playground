run:
    cargo run --locked --release -p component-playground-desktop

build:
    cargo build --locked --release -p component-playground-desktop

check:
    cargo check --locked -p component-playground-ui -p component-playground-desktop

dev:
    cargo run --locked -p component-playground-desktop

test:
    cargo test --locked --workspace

lint:
    cargo clippy --locked --workspace --all-targets

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

release-check: fmt-check check test lint build

release VERSION: release-check
    #!/usr/bin/env bash
    set -euo pipefail
    project_version="$(cargo metadata --no-deps --format-version 1 | python3 -c 'import json, sys; print(next(package["version"] for package in json.load(sys.stdin)["packages"] if package["name"] == "component-playground-desktop"))')"
    if [[ "$project_version" != "{{VERSION}}" ]]; then
        echo "Cargo version ($project_version) does not match requested release ({{VERSION}})" >&2
        exit 1
    fi
    if [[ -n "$(git status --porcelain)" ]]; then
        echo "release requires a clean working tree" >&2
        exit 1
    fi
    tag="v{{VERSION}}"
    if git rev-parse "$tag" >/dev/null 2>&1; then
        echo "tag already exists: $tag" >&2
        exit 1
    fi
    git tag -a "$tag" -m "Release $tag"
    git push origin "$tag"
