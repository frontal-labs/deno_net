# Notice

This crate is a standalone copy of `ext/net` from
[denoland/deno](https://github.com/denoland/deno) at tag `v2.9.7`
(`deno_net` 0.251.0), MIT licensed. See `LICENSE.md`.

Changes from upstream:

- Workspace dependency versions are pinned explicitly, because the crate is
  built outside the Deno monorepo.
- `deno_tls` resolves to the patched `deno_tls` crate (rustls `=0.23.45`).
  The sandbox workspace applies this through `[patch.crates-io]`.
- No code changes yet. The hickory 0.25 to 0.26 migration is still to do.
