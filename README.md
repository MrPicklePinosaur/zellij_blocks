<div align="center">

# zellij_blocks

[![crates.io](https://img.shields.io/crates/v/zellij_blocks.svg)](https://crates.io/crates/zellij_blocks)
[![docs.rs](https://docs.rs/zellij_blocks/badge.svg)](https://docs.rs/zellij_blocks)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

## SETTING UP FOR DEVELOPMENT

Ensure that you have `wasm32-wasi` as a rust build target:
```sh
rustup target add wasm32-wasi
```

Install git hooks
```sh
just devsetup
```

Run a developer workspace to debug the plugin
```sh
zellij --layout ./plugin-dev-workspace.kdl
```

or from a current zellij session
```sh
zellij action new-tab --layout ./plugin-dev-workspace.kdl
```

