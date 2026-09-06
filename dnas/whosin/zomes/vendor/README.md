# Vendored zome crates

Code in this directory is **not** first-party to whos-in. It is a verbatim copy of a
third-party zome crate, kept here only because the upstream project has no Holochain 0.7
release to depend on.

## `notifications_coordinator` (package `hc_zome_notifications_coordinator`)

Source: <https://github.com/holochain-open-dev/notifications>, branch `main-0.6`,
commit `f3b63da5ebeb02b2b40f8238718ecb958f27288c` ("Update to Holochain 0.6", 2025-12-08),
path `zomes/coordinator/notifications/`.

The matching integrity crate (`zomes/integrity/notifications/` upstream) was vendored into
`dnas/whosin/zomes/integrity/notifications/`, which used to be a one-line
`extern crate hc_zome_notifications_integrity;` wrapper around it.

### Why

`git ls-remote https://github.com/holochain-open-dev/notifications` lists **no tags at all**
and no `main-0.7` branch (checked 2026-09-06; branches are `0.2-merge`, `0.3`, `0.3.1`,
`0.4`, `gh-pages`, `main`, `main-0.4`, `main-0.5`, `main-0.6`). There is therefore nothing
to point a git dependency at on the 0.7 line, and Global Constraints forbids depending on a
branch. The only fork, `ddd-mtl/notifications`, was last pushed in 2024.

### What changed relative to upstream

The `src/` trees are byte-identical to upstream at the commit above **as vendored**; the
Holochain 0.7 migration was then applied on top, in the same commits as the rest of the
0.7 upgrade, so `git log -p` over this directory is the full delta. The only Cargo.toml
changes are the dependency-source lines (git/relative-path -> workspace members).

### Going back to upstream

If `holochain-open-dev/notifications` ever publishes a 0.7 tag, the swap is:

1. delete this directory and restore `dnas/whosin/zomes/integrity/notifications/src/lib.rs`
   to `extern crate hc_zome_notifications_integrity;`;
2. restore the two `git = ...` dependency lines in the two zome `Cargo.toml`s;
3. drop the `dnas/*/zomes/vendor/*` workspace member glob and the
   `hc_zome_notifications_coordinator` workspace dependency.

**Do not do this after the canonical happ has been frozen.** The integrity zome is hashed
into the DNA, so any byte difference between the vendored source and the upstream tag is a
new network.
