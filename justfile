run:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ "$(uname -s)" == "Darwin" ]]; then
      cargo build --locked --release -p component-playground-desktop
      app_path="$(scripts/create-macos-app.sh release)"
      "$app_path/Contents/MacOS/Component Playground"
    else
      cargo run --locked --release -p component-playground-desktop
    fi

build:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build --locked --release -p component-playground-desktop
    if [[ "$(uname -s)" == "Darwin" ]]; then
      scripts/create-macos-app.sh release >/dev/null
    fi

check:
    cargo check --locked -p component-playground-ui -p component-playground-desktop

dev:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ "$(uname -s)" == "Darwin" ]]; then
      cargo build --locked -p component-playground-desktop
      app_path="$(scripts/create-macos-app.sh debug)"
      "$app_path/Contents/MacOS/Component Playground"
    else
      cargo run --locked -p component-playground-desktop
    fi

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
