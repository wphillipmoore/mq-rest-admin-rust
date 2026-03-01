# LLVM Coverage Instrumentation Notes

## LLVM 18 Phantom Uncovered Regions (Rust 1.92)

### Summary

Rust 1.92 ships with LLVM 18, which has a coverage instrumentation bug that
reports phantom uncovered regions for certain code patterns. The same code
compiled with Rust 1.93 (LLVM 19) reports 100% coverage. All tests pass on
both versions -- the issue is purely in coverage accounting, not in actual
test gaps.

### Affected Patterns

The following code patterns produce phantom uncovered regions on LLVM 18 but
not LLVM 19:

- **`const fn` accessors** on structs -- simple field-returning `const fn`
  methods generate uncoverable instrumentation points.
- **`format!` macro in error paths** -- the `format!` expansion creates
  internal branches that LLVM 18 marks as separate regions, some of which
  are reported as uncovered even when the containing function is exercised.
- **`?` operator in validation functions** -- the desugared `match` from `?`
  sometimes produces phantom uncovered regions in the `Ok` arm.
- **Helper functions returning `Result`** -- even when fully tested through
  both `Ok` and `Err` paths, LLVM 18 may report 1-2 uncovered regions per
  helper.

### Workarounds Attempted

During the `SyncConfig` validation work (issue #42), the following
approaches were tried to achieve 100% coverage on Rust 1.92. **None
succeeded** -- all produced exactly 4 phantom uncovered regions:

1. Removed `const fn`, added `#[allow(clippy::missing_const_for_fn)]`
2. Extracted a `validate_positive` helper function
3. Restructured with combined `if/else` for field/value selection
4. Made struct fields public (removing accessor methods entirely)
5. Used `if let Err(e)` instead of `?` with `#[allow(clippy::question_mark)]`
6. Replaced `format!` with `String::from` + `push_str`
7. Various combinations of the above

### Resolution

The CI matrix uses per-version coverage thresholds:

- **Rust 1.93 (LLVM 19)**: 100% lines, 100% regions
- **Rust 1.92 (LLVM 18)**: 99% lines, 99% regions

This accommodates the LLVM 18 instrumentation bug while maintaining strict
coverage enforcement on the current LLVM version. When Rust 1.92 is dropped
from the matrix, the relaxed threshold goes with it.

### References

- PR: #43 (SyncConfig construction validation)
- Issue: #42
- Upstream: This is a known class of LLVM 18 coverage instrumentation issue
  resolved in LLVM 19.
