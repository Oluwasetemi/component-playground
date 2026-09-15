# Component Playground

Native [GPUI](https://www.gpui.rs/) component gallery using Longbridge's
GPUI Kit component library and the Groknight dark theme.

## Workspace

```
crates/
  ui/        # Theme, stories, and root view (`component-playground-ui`)
  desktop/   # Native binary bootstrap (`component-playground-desktop`)
```

## Run

```sh
just dev          # debug
just run          # release
just check        # typecheck
just test         # workspace tests
just lint         # Clippy
```

Or:

```sh
cargo run --locked --release -p component-playground-desktop
```

Quit with **⌘Q** / **Ctrl+Q**.

## Stack

- Longbridge `gpui-kit` facade and `gpui-component` library
- Groknight theme colors (`crates/ui/themes/groknight.json`)

## Releases

Release tags use SemVer, including prereleases:

```text
v0.1.0-alpha.1
v0.1.0-beta.1
v0.1.0
```

Before creating a tag, run the complete local release validation and then use
the release recipe. The argument must match `[workspace.package] version` in
`Cargo.toml`:

```sh
just release-check
just release 0.1.0
```

The tag workflow validates the Cargo version, builds locked macOS ARM64,
macOS Intel, Linux x86_64, and Windows x86_64 binaries, publishes archives and
SHA-256 checksums, and creates the GitHub Release. Signing, installer/DMG
packaging, and additional CPU architectures can be added when the application
is ready for wider distribution.
