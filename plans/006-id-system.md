# ID System

Implement `tomb-cli/src/id.rs` — task ID generation, prefix resolution, assignment, and duplicate detection.

Prerequisite: 001-cargo-root.md (error types and model types with `Task.id`)

## 1. Module scaffolding

- [x] Create `tomb-cli/src/id.rs`
- [x] Add `pub mod id;` to `lib.rs`
- [x] Add `IdNotFound` and `AmbiguousId` error variants to `TombError`
- [x] Verify: `cargo check -p tomb-cli`

## 2. ID generation — `generate_id(existing)`

- [x] Generate a random 4-character alphanumeric string `[a-z0-9]`
- [x] Accept a `HashSet<&str>` of existing IDs
- [x] Retry on collision (loop until unique)
- [x] Use `rand` crate for randomness

## 3. Prefix resolution — `resolve_prefix(prefix, all_ids)`

- [x] If `prefix` is an exact match, return it
- [x] If `prefix` matches exactly one ID as a prefix, return the full ID
- [x] If ambiguous (multiple matches), return `TombError::AmbiguousId` with the list of matches
- [x] If no match, return `TombError::IdNotFound`

## 4. ID collection — `collect_ids(file)`

- [x] Walk all tasks recursively (including children at all depths)
- [x] Return `Vec<&str>` of all IDs found
- [x] Skip tasks with `id: None`

## 5. Assign missing IDs — `assign_missing_ids(file)`

- [x] Collect all existing IDs into a `HashSet`
- [x] Walk all tasks recursively
- [x] For any task with `id: None`, generate a new unique ID and set it
- [x] Return count of IDs assigned

## 6. Duplicate detection — `find_duplicates(file)`

- [x] Walk all tasks, collect IDs with their locations
- [x] Return a `Vec` of `(id, count)` for any ID appearing more than once

## 7. Duplicate repair (used by sync)

- [x] Keep the first occurrence of each duplicated ID
- [x] Reassign new IDs to subsequent occurrences
- [x] Return count of IDs reassigned

## 8. Tests

- [x] `generate_id` produces 4-char alphanumeric strings
- [x] `generate_id` avoids collisions with existing set
- [x] `resolve_prefix` exact match works
- [x] `resolve_prefix` unique prefix match works
- [x] `resolve_prefix` ambiguous prefix returns error with matches
- [x] `resolve_prefix` no match returns `IdNotFound`
- [x] `assign_missing_ids` assigns to tasks without IDs, skips those with IDs
- [x] `find_duplicates` detects dupes, returns empty when none
- [x] Duplicate repair keeps first, reassigns rest

## Verify

- [x] `cargo test -p tomb-cli -- id` — all ID tests pass
