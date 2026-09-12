# Component Playground

Native [GPUI](https://www.gpui.rs/) component gallery using Longbridge's
component library and the Groknight dark theme.

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

- GPUI + `gpui_platform` from [Zed](https://github.com/zed-industries/zed)
- Longbridge `gpui-component`
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

The tag workflow validates the Cargo version, builds locked macOS ARM64 and
Intel binaries, publishes archives and SHA-256 checksums, and creates the
GitHub Release. Signing, DMG packaging, and additional platforms can be added
when the application is ready for wider distribution.
