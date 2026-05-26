# AGENTS.md

Guidance for AI coding agents (Copilot, Claude, Codex, Cursor, etc.) working
in the `openDevicePartnership/embedded-batteries` repository. Human
contributors are welcome to read it too — everything here is a verified
description of how the repo actually works.

Read this file end-to-end before making any change. Then prefer small,
surgical edits that match the patterns already in the tree.

---

## What this crate is

`embedded-batteries` is a Hardware Abstraction Layer (HAL) for **battery
fuel gauges** and **battery chargers** used in embedded systems. It is
hardware- and platform-independent: the workspace defines *traits*, not
drivers.

- The blocking traits follow the
  [Smart Battery System v1.1 (SBS) specification](https://sbs-forum.org/specs/sbdat110.pdf).
- ACPI Power Source / Power Meter device messages are defined as a
  superset of SBS for users building ACPI-compliant systems
  (see the [ACPI 6.4 spec, chapter 10](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/10_Power_Source_and_Power_Meter_Devices/Power_Source_and_Power_Meter_Devices.html)).
- Both a blocking and an async flavor are provided as **separate crates**
  in the same workspace; the async crate re-uses ACPI types from the
  blocking crate.

Concrete fuel-gauge and charger drivers live in *other* repositories and
implement these traits. **This repo intentionally contains no device
drivers.** Do not add one.

---

## Workspace layout

```
.
├── Cargo.toml                  # virtual workspace, resolver = "2"
├── README.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── CODEOWNERS
├── SECURITY.md
├── LICENSE                     # MIT
├── deny.toml                   # cargo-deny configuration
├── rustfmt.toml                # nightly-only options (see below)
├── .github/
│   ├── copilot-instructions.md # commit-message + AI-attribution rules
│   └── workflows/
│       ├── check.yml           # fmt, clippy, doc, hack, deny, msrv, semver
│       └── nostd.yml           # currently a duplicate of check.yml
├── embedded-batteries/         # blocking crate (published on crates.io)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # shared types + module pub uses
│       ├── charger.rs          # blocking `Charger` trait + error types
│       ├── smart_battery.rs    # SBS traits, bitfields, wrapper macro
│       └── acpi.rs             # ACPI BIX/BST/BIF/BMC/... types
└── embedded-batteries-async/   # async crate (published on crates.io)
    ├── Cargo.toml
    └── src/
        ├── lib.rs              # re-exports ACPI from the blocking crate
        ├── charger.rs          # async `Charger` trait
        └── smart_battery.rs    # async SBS trait + wrapper macro
```

### Member crates

| Crate                       | Version | Description                                                                                |
|-----------------------------|---------|--------------------------------------------------------------------------------------------|
| `embedded-batteries`        | 0.3.4   | Blocking SBS + ACPI traits and types. Owns the shared `MilliAmps`/`MilliVolts` aliases.    |
| `embedded-batteries-async`  | 0.3.4   | Async mirror of the SBS traits. Re-exports `embedded_batteries::acpi` and charger errors.  |

Both crates are `no_std`, MIT-licensed, MSRV **1.83**, edition 2021,
optional `defmt` feature.

### Shared types (defined in `embedded-batteries/src/lib.rs`)

```rust
pub type MilliAmps = u16;
pub type MilliVolts = u16;
pub type MilliAmpsSigned = i16;
pub type MilliVoltsSigned = i16;
```

The async crate uses these via `pub use embedded_batteries::{MilliAmps, MilliVolts};`.
**Do not duplicate them** in the async crate.

---

## Building & testing (verified commands)

All commands below were executed at the tip of `upstream/main` while
authoring this file. Cargo commands run from the workspace root.

| Command                                                                                                              | Status | Notes                                                                 |
|----------------------------------------------------------------------------------------------------------------------|--------|-----------------------------------------------------------------------|
| `cargo check --all-features`                                                                                         | ✅     | Builds both crates with `defmt` on.                                   |
| `cargo fmt --check`                                                                                                  | ✅     | See note about nightly below.                                         |
| `cargo clippy --all-features --all-targets -- -F clippy::suspicious -F clippy::correctness -F clippy::perf -F clippy::style` | ✅ (with warnings) | Same lint set the CI clippy-action passes. Emits `bitflags`-related future-incompat warnings; **do not** silence these locally. |
| `cargo doc --no-deps --all-features`                                                                                 | ✅ (1 warning) | Pre-existing `rustdoc::broken_intra_doc_links` warning in `acpi.rs` near `Status Flag bit [3]`. Leave it alone unless your change touches that doc-comment. |
| `cargo test`                                                                                                         | ✅     | Only doctests exist (currently `ignored`).                            |
| `cargo test --all-features`                                                                                          | ❌     | Fails when linking host tests with `defmt` enabled (`_defmt_acquire` & friends are unresolved). CI does not run this and neither should you. |
| `cargo hack --feature-powerset check`                                                                                | (CI)   | Runs in CI; install with `cargo install cargo-hack` if you need it locally. |
| `cargo deny --all-features check`                                                                                    | (CI)   | License/advisory/source/ban check. Config in `deny.toml`.             |
| `cargo semver-checks`                                                                                                | (CI)   | Enforces semver on public API changes.                                |

### rustfmt and nightly

`rustfmt.toml` enables two **unstable** options:

```toml
group_imports = "StdExternalCrate"
imports_granularity = "Module"
max_width = 120
```

On stable, `cargo fmt --check` prints warnings ("unstable features are
only available in nightly channel") **and still exits 0**. CI installs
the nightly toolchain solely to run `cargo fmt --check`. If you reformat
imports locally, use `cargo +nightly fmt` so the import grouping/granularity
options actually take effect.

### MSRV

CI builds with `1.83`. Do not introduce APIs newer than that. The MSRV
notes in `.github/workflows/check.yml` track *why* the floor exists
(`bitfield-struct` requires 1.83, etc.).

---

## Code conventions

These are mined from the existing code; follow them when adding to it.

### Trait shape (mirrors `embedded-hal`)

Every functional trait family has the same five-piece anatomy:

1. An `Error` trait extending `core::fmt::Debug`, with a single
   `fn kind(&self) -> ErrorKind`.
2. A `#[non_exhaustive]` `ErrorKind` enum, `#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]`
   and `#[cfg_attr(feature = "defmt", derive(defmt::Format))]`.
3. `impl Error for core::convert::Infallible` (with `match *self {}`).
4. `impl Error for ErrorKind` returning `*self`.
5. `core::fmt::Display` for the error kind.
6. An `ErrorType` trait carrying the associated `Error` type, plus a
   blanket `impl<T: ErrorType + ?Sized> ErrorType for &mut T`.

Then the functional trait (`Charger`, `SmartBattery`, …) extends
`ErrorType` and gets a `&mut T` forwarding `impl`. Look at
`embedded-batteries/src/charger.rs` for the canonical 100-line template.

### Async traits

Async traits use **explicit `impl Future<Output = …>` return types** in
the trait definition and `async fn` only in `impl` blocks:

```rust
pub trait Charger: ErrorType {
    fn charging_current(&mut self, current: MilliAmps)
        -> impl Future<Output = Result<MilliAmps, Self::Error>>;
}

impl<T: Charger + ?Sized> Charger for &mut T {
    #[inline]
    async fn charging_current(&mut self, current: MilliAmps) -> Result<MilliAmps, Self::Error> {
        T::charging_current(self, current).await
    }
}
```

Preserve this style; don't switch the trait declarations to `async fn`.

### `&mut T` blanket impls are intentionally *the only* blanket impls

PR #34 ("Remove blanket impl for &mut T and add wrapper macro") removed
the `impl<T: ?Sized + …>` blanket for non-`&mut` references and replaced
it with the `impl_smart_battery_for_wrapper_type!` macro for wrappers
that need their own impl (e.g. `Mutex`-wrapped batteries). When you need
to wrap a `SmartBattery` in your own type, **use the macro** rather than
adding a new blanket impl. The macro is exported from both
`embedded-batteries::smart_battery` and
`embedded-batteries-async::smart_battery`.

### Module documentation pattern

Both `lib.rs` files start with:

```rust
#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]
```

Anything `pub` must have a doc comment, or `#![warn(missing_docs)]` will
flag it. New ACPI types in particular should document the value ranges
(`0x00000000..=0x7FFFFFFF: Valid`, `0xFFFFFFFF: Unknown`, …) the way
existing types do.

### `defmt` feature

- Optional in both crates: `defmt = ["dep:defmt"]` in the blocking
  crate, `defmt = ["dep:defmt", "embedded-batteries/defmt"]` in the
  async crate. Keep features **additive** — `cargo hack --feature-powerset check`
  in CI will fail otherwise.
- Gate every `defmt::Format` derive behind
  `#[cfg_attr(feature = "defmt", derive(defmt::Format))]`. Never make
  `defmt` a hard dependency.

### Bitfields & wire types

- Register-style structs use `bitfield_struct::bitfield` (see
  `smart_battery.rs`).
- ACPI wire structs use `bitflags!` for flag fields and `zerocopy`
  (`FromBytes`, `IntoBytes`, `Immutable`) for byte-level layout (see
  `acpi.rs`). When adding new ACPI structs, follow the same derives and
  define a `pub const FOO_RETURN_SIZE_BYTES: usize` constant alongside.

### Numeric types

Use the workspace aliases (`MilliAmps`, `MilliVolts`, `MilliAmpsSigned`,
`MilliVoltsSigned`) instead of raw `u16`/`i16` in public signatures.
Document the unit (`mA`, `mV`, `mAh`, `mWh`) for any new field.

### Rustfmt details

- `max_width = 120`.
- Nightly: imports grouped `StdExternalCrate` with `Module` granularity.
  After adding imports, run `cargo +nightly fmt` so the resulting
  ordering matches CI.

---

## Driver-or-HAL specifics

This crate is the **HAL side**. Things you should *not* do here:

- Don't add an `I2C`/`SPI` dependency. Implementations live in driver
  crates and depend on `embedded-hal` themselves.
- Don't add a `std`-using example or test. The crates are `no_std`; CI
  has no host integration tests.
- Don't add concrete chip drivers (BQ25xxx, MAX17xxx, etc.).
- Don't take a hardware dependency on a specific bus.
- Don't break `cargo semver-checks`. Any change that alters a public
  signature must be paired with a version bump in the corresponding
  `Cargo.toml`; the CI semver job will fail otherwise.

Things you *should* do:

- Add new SBS or ACPI methods to the trait, plus a forwarding
  `impl<T: Trait + ?Sized> Trait for &mut T` body.
- Mirror any addition to the blocking trait in the async trait (and the
  wrapper macro, if applicable).
- Keep ACPI types in `embedded-batteries/src/acpi.rs`. The async crate
  re-exports them via `pub use embedded_batteries::acpi;` — adding ACPI
  code to the async crate would create a duplicate definition.

---

## Commit & PR conventions

Verified by skimming `git log --pretty=%s upstream/main | head -30`:

- Subjects are short, imperative, capitalized (e.g. *"Add Debug derive
  for ACPI types"*, *"Fix missed ? operator in macro"*, *"Remove
  blanket impl for &mut T and add wrapper macro"*).
- One topic per branch; PRs typically map 1:1 to a topic branch like
  `acpi-improvements`, `add-clippy-allow-and-uprev-bitfield-struct`.
- Squash-merging is **disabled**. From `CONTRIBUTING.md`:
  > Each commit builds successfully without warning
  > Miscellaneous commits to fix typos + formatting are squashed
  Rebase/reorder before opening the PR; do not rely on the merge button
  to flatten history.
- PR etiquette:
  - Open as **draft** first.
  - Confirm the `.github/` workflows pass on your draft before marking
    it ready for review.
- Regressions should be reported with the offending commit identified
  via `git bisect`.

### Commit message format (from `.github/copilot-instructions.md`)

- Subject line: capitalized, **≤ 50 chars**, imperative mood ("Fix bug"
  not "Fixed bug").
- Blank line between subject and body.
- Wrap body at **72 columns**.
- Body explains the *what* and *why*, not the *how*.

### AI attribution (mandatory)

Every commit that contains AI-generated or AI-assisted work **must**
end with an `Assisted-by` trailer:

```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```

- `AGENT_NAME` — e.g. `GitHub Copilot`.
- `MODEL_VERSION` — the *actual* model that produced the work. **Verify
  this for the current session — do not copy a value from another
  commit.**
- Optional `[TOOLn]` — specialized analysis tools used (coccinelle,
  sparse, smatch, clang-tidy, …). Basic tools (git, cargo, editor) are
  not listed.

Agents **must not** add `Signed-off-by` trailers; only humans can
certify the DCO.

---

## What not to do

- Don't add a concrete device driver.
- Don't add `std`, host-only test harnesses, or examples that require a
  bus.
- Don't introduce features that aren't strictly additive (`cargo hack
  --feature-powerset check` will catch you).
- Don't replace `impl<T: ?Sized> Trait for &mut T` with a blanket impl
  over a broader bound. PR #34 removed those on purpose.
- Don't `cargo update` to bump `Cargo.lock` — there is no committed
  `Cargo.lock` (`.gitignore` excludes it; this is a library workspace).
- Don't bump MSRV without updating `rust-version` in both crate
  manifests *and* the `msrv` matrix in `.github/workflows/check.yml`.
- Don't silence the existing `bitflags` future-incompat warnings or the
  one `rustdoc::broken_intra_doc_links` warning in `acpi.rs` unless your
  change is specifically about fixing them.
- Don't run `cargo test --all-features` and try to "fix" the resulting
  link errors — they are an artifact of building `defmt` for the host
  target with no `defmt` global logger. CI does not run this command.
- Don't add a `Signed-off-by:` trailer (see AI attribution rules).
- Don't force-push shared branches.

---

## Context

- **License:** MIT. Contributions are accepted under the same terms
  unless you flag otherwise in the PR (see `CONTRIBUTING.md`).
- **Maintainers:** see `CODEOWNERS`.
- **Security:** report via `SECURITY.md`; do not open public issues for
  vulnerabilities.
- **Code of conduct:** `CODE_OF_CONDUCT.md`.
- **CI:** `.github/workflows/check.yml` (and a near-identical
  `nostd.yml`). Both run on every PR and on pushes to `main`. The
  `check` workflow runs every job **per commit in the PR**, so each
  commit must build, format, lint, and document cleanly — not just the
  PR tip.
- **Line endings:** repository convention is **LF**. The local clone
  has `core.autocrlf=false`; do not re-enable autocrlf and do not let
  your editor rewrite files with CRLF.
- **`Cargo.lock`:** intentionally untracked — this is a library
  workspace.

---

## Incorporated from `.github/copilot-instructions.md`

The repository's `copilot-instructions.md` is the authoritative source
for commit-message rules and AI-attribution rules. This file restates
those rules in the *Commit & PR conventions* section above so that
single-file agents (Codex, Cursor, etc.) that read only `AGENTS.md`
still get them. If the two files ever disagree, `copilot-instructions.md`
wins; please update `AGENTS.md` to match in the same commit.

The pointer note now at the top of `copilot-instructions.md` directs
Copilot users back here for everything that is *not* about commit
messages.

## Model selection & cost discipline

Premium models (Opus, GPT-5 family, "high"/"xhigh" reasoning variants)
cost an order of magnitude more than standard models (Sonnet, Haiku,
mini). Most steps in a typical task do not need premium reasoning,
and over-using premium models wastes credits without improving
outcomes. The rules below apply to *all* model selection: your own
session, sub-agents launched via the `task` tool, and parallel work
launched via `/fleet`.

### Default posture

- **Default to the cheapest model that can do the job.** Reach for a
  premium model only when one of the escalation triggers below is hit.
- **Plan with premium, execute with cheap.** Spend at most one or two
  premium turns on design / planning, then downshift to a cheaper
  model for mechanical execution of the plan.
- **Never bump the model "just in case."** If you cannot articulate
  *why* a cheaper model would fail, use the cheaper model.

### Escalation triggers (use a premium model)

Reach for a premium model when *any* of these are true:

- Cross-module refactor, architectural design, or API design from
  scratch.
- Subtle correctness reasoning: concurrency, lifetimes, `unsafe`,
  FFI ABI, cryptography, safety-critical control paths.
- Debugging a failure that survived one prior cheap-model attempt.
- Reviewing code on a safety-, security-, or money-critical path.
- The diff cannot be predicted in advance — i.e. there is genuine
  creative or design work to do, not just typing.

### De-escalation triggers (use a cheap model)

Use the cheapest available model when *any* of these are true:

- Searching, reading, summarising files or docs.
- Single-file mechanical edits: rename, format, lint fix, dependency
  bump, boilerplate, scaffolding from a known template.
- Generating tests for code that already works.
- Running builds, tests, linters, or other commands where the model
  only needs to report success/failure.
- Routine commits, PR descriptions, changelog entries.
- The diff is essentially predictable before generation.

### Sub-agent routing (the `task` tool)

When delegating with the `task` tool, set `model:` explicitly. Do not
let sub-agents inherit a premium default for cheap work.

| Sub-agent type    | Default model             | Override to                                     |
|-------------------|---------------------------|-------------------------------------------------|
| `explore`         | cheap                     | keep cheap (`claude-haiku-4.5` or `gpt-5-mini`) |
| `task` (run cmd)  | cheap                     | keep cheap                                      |
| `research`        | cheap for breadth         | premium only for the final synthesis            |
| `general-purpose` | match task                | cheap for mechanical work; premium for design   |
| `rubber-duck`     | premium                   | keep premium — this is where reasoning pays off |
| `code-review`     | premium on critical paths | cheap on cosmetic / mechanical diffs            |

### `/fleet` (parallel sub-agents) rules

- Fleet mode multiplies cost by the fleet width. Apply the rules
  above *per worker*, not in aggregate.
- Split a fleet job along complexity lines: route the cheap,
  parallelisable workers (file edits, test runs, doc updates) to a
  cheap model; reserve premium models for the small number of
  workers that need real reasoning.
- If every worker in a fleet would need a premium model, the work is
  probably not a good fit for fleet mode — reconsider the
  decomposition before paying N× premium.

### Session hygiene

- Keep sessions short and focused. Long premium sessions are the
  single largest source of waste because every turn re-processes the
  full history.
- Use `/compact` when the conversation grows long, and `/new` for
  unrelated work.
- Prefer `/ask` for one-off side questions so they don't extend the
  main session.

### When in doubt

Ask: *"If a cheaper model produced the wrong answer here, would I
catch it in seconds (compiler, tests, my own review) or in
weeks (production incident)?"* If the former, use the cheap model
and let the feedback loop do its job.
