# Sigma System Foundation

Small Rust library and binary for holding system-adjustment preferences and a
selected desktop persona. The library exposes `SystemAdjuster` and
`PersonaManager`; the current binary prints their in-memory values.

## Current behavior and limits

`SystemAdjuster::generate_sysctl_conf()` formats two sysctl settings, but the
program does not write that file or change a CPU governor. `PersonaManager`
stores a selection in memory; it does not install themes, packages, or desktop
configuration. The binary's output is a report of configured values, not proof
that system adjustments were applied.

Keep proposed settings separate from applying them. A future apply path should
show the exact changes, require appropriate privileges, preserve prior values,
and provide rollback and evidence that the operating system accepted them.

## Development

The crate has no third-party dependencies. Run `cargo check` to compile and
`cargo test` to run its unit tests. Omarchy and Linux Mint are references for
workflow and desktop defaults; this crate does not reproduce their system
configuration behavior.
