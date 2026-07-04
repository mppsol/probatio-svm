# vendor/hermit-abi — offline-build shim (not a real dependency)

This is a **no-op stub**, not the real [`hermit-abi`](https://crates.io/crates/hermit-abi) crate.

`num_cpus` (pulled in transitively by the litesvm/solana test stack) lists `hermit-abi` as a
dependency, but it is only ever compiled under `target_os = "hermit"` (the Hermit unikernel). This
project builds only for the host (macOS/Linux) and the Solana BPF target, so **hermit-abi's code is
never compiled here**.

hermit-abi 0.5.2 is not present in the local cargo registry cache, so an offline resolve
(`cargo build --offline`) fails even though the crate would never be compiled. The workspace
`[patch.crates-io]` in the root `Cargo.toml` points `hermit-abi` at this empty `#![no_std]` stub so
offline resolution succeeds. Because the crate is never compiled on any target we use, the stub is
functionally safe.

**Remove this** once the real crate is cached locally or builds run online — then delete the
`[patch.crates-io]` entry in the root `Cargo.toml`.
