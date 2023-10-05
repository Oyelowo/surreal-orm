## Description

This PR introduces various enhancements and new features to the `surreal-orm`
query builder. A slew of new statement types have been implemented, offering a
richer query construction layer. The changes ensure that the library is more
extensible, maintainable, and user-friendly.

### Features

1. **Statement Support**: New support for BREAK, CONTINUE, LIVE SELECT, REMOVE
   ANALYZER, REMOVE FUNCTION, REMOVE PARAM, and SHOW CHANGES statements.
2. **Query Building Improvements**: Changes in
   `query-builder/src/functions/search.rs` to improve query build logic.
3. **Code Cleanup**: Refactoring in various files to improve readability and
   maintainability.

### Changes

1. `query-builder/src/functions/search.rs`: Renamed `search_highlight_fn` to
   `highlight_fn` and updated corresponding tests.
2. `query-builder/src/statements/for_loop.rs`: Various formatting and naming
   changes for better readability.
3. Added new statement files: `break_.rs`, `continue_.rs`, `live_select.rs`,
   `remove_analyzer.rs`, `remove_function.rs`, `remove_param.rs`, `show.rs`
4. `query-builder/src/statements/mod.rs`: Included the newly added statement
   types in the module tree and public API.

### Test Coverage

- Added unit tests for all new statements.
- Updated existing unit tests in `search.rs` and `for_loop.rs` to reflect the
  changes.

### Backward Compatibility

- All changes are backward compatible.

## Checklist

- [x] Code compiles correctly
- [x] Created new unit tests for the features
- [x] All unit tests passing
- [x] Extended the README / documentation if necessary
- [x] Applied code formatting

## Reviewers

- [ ] @Reviewer1
- [ ] @Reviewer2

---

### Files Changed

- `query-builder/src/functions/search.rs`
- `query-builder/src/statements/break_.rs`
- `query-builder/src/statements/continue_.rs`
- `query-builder/src/statements/for_loop.rs`
- `query-builder/src/statements/live_select.rs`
- `query-builder/src/statements/remove_analyzer.rs`
- `query-builder/src/statements/remove_function.rs`
- `query-builder/src/statements/remove_param.rs`
- `query-builder/src/statements/show.rs`
- `query-builder/src/statements/mod.rs`
