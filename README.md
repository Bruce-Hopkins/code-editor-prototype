# Code Editor Prototype

Currently, this editor is in pre-alpha. Missing features and bugs are expected.

## Description

This is a code editor GUI editor built with Rust and Iced. The goal is to become a highly customizablizable, lightweight editor. With a built in highlighter and LSP support.

Currently, this is only a demo. The editor only highlights Rust, and the LSP client only works with Rust analyzer. Better support for other programming languages will be supported in the future.

## Build

> For Linux users, `libgtk` will need to be installed to build the project.

Inside the root of the codebase run

```bash
cargo run --release
```

To receive diagnostics from rust analyzer, install the rust analyzer binary to your $PATH with [these instructions](https://rust-analyzer.github.io/manual.html#rust-analyzer-language-server-binary).

## Todo

While the current state of the editor is a great starting point, there's a lot of work that needs to be accomplished before this can be a fully-fledged code editor. That includes:

- [ ] Adding a scripting language to allow for plugins and extensions.
- [ ] Upgrading to Iced 0.12
- [ ] Adding a file explorer
- [ ] Improving LSP support
- [ ] Supporting more programming languages
