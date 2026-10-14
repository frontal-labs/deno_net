# Notice

This crate is a standalone copy of `ext/net` from
[denoland/deno](https://github.com/denoland/deno) at tag `v2.9.7`
(`deno_net` 0.251.0), MIT licensed. See `LICENSE.md`.

Changes from upstream:

- Workspace dependency versions are pinned explicitly, because the crate is
  built outside the Deno monorepo.
- `hickory-proto` and `hickory-resolver` are 0.26 (upstream: 0.25). `ops.rs`
  is ported to the 0.26 API: the error types, the runtime provider, the
  name-server configuration, the lookup accessors, and the record fields.
- `system_conf.rs` is new. On Apple platforms it reads `/etc/resolv.conf`
  instead of hickory's SystemConfiguration loader, which rejects zone-scoped
  IPv6 DNS servers such as `fe80::1%en0`. This keeps `resolveDns` working on
  those networks, as it did with hickory 0.25. Its conversion is adapted from
  hickory-resolver `src/system_conf/unix.rs`, which is Apache-2.0 OR MIT.
- `deno_tls` resolves to the patched `deno_tls` crate (rustls `=0.23.45`).
  The sandbox workspace applies this through `[patch.crates-io]`.
- Test fixtures from `ext/tls/testdata` are copied into `testdata/`.
