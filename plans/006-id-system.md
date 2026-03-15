# ID System

Implement `tomb-cli/src/id.rs` — task ID generation, prefix resolution, assignment, and duplicate detection.

Prerequisite: 001-cargo-root.md (error types and model types with `Task.id`)

## 1. Module scaffolding

- [ ] Create `tomb-cli/src/id.rs`
- [ ] Add `pub mod id;` to `lib.rs`
- [ ] Add `IdNotFound` and `AmbiguousId` error variants to `TombError`
- [ ] Verify: `cargo check -p tomb-cli`

## 2. ID generation — `generate_id(existing)`

- [ ] Generate a random 4-character alphanumeric string `[a-z0-9]`
- [ ] Accept a `HashSet<&str>` of existing IDs
- [ ] Retry on collision (loop until unique)
- [ ] Use `rand` crate for randomness

## 3. Prefix resolution — `resolve_prefix(prefix, all_ids)`

- [ ] If `prefix` is an exact match, return it
- [ ] If `prefix` matches exactly one ID as a prefix, return the full ID
- [ ] If ambiguous (multiple matches), return `TombError::AmbiguousId` with the list of matches
- [ ] If no match, return `TombError::IdNotFound`

## 4. ID collection — `collect_ids(file)`

- [ ] Walk all tasks recursively (including children at all depths)
- [ ] Return `Vec<&str>` of all IDs found
- [ ] Skip tasks with `id: None`

## 5. Assign missing IDs — `assign_missing_ids(file)`

- [ ] Collect all existing IDs into a `HashSet`
- [ ] Walk all tasks recursively
- [ ] For any task with `id: None`, generate a new unique ID and set it
- [ ] Return count of IDs assigned

## 6. Duplicate detection — `find_duplicates(file)`

- [ ] Walk all tasks, collect IDs with their locations
- [ ] Return a `Vec` of `(id, count)` for any ID appearing more than once

## 7. Duplicate repair (used by sync)

- [ ] Keep the first occurrence of each duplicated ID
- [ ] Reassign new IDs to subsequent occurrences
- [ ] Return count of IDs reassigned

## 8. Tests

- [ ] `generate_id` produces 4-char alphanumeric strings
- [ ] `generate_id` avoids collisions with existing set
- [ ] `resolve_prefix` exact match works
- [ ] `resolve_prefix` unique prefix match works
- [ ] `resolve_prefix` ambiguous prefix returns error with matches
- [ ] `resolve_prefix` no match returns `IdNotFound`
- [ ] `assign_missing_ids` assigns to tasks without IDs, skips those with IDs
- [ ] `find_duplicates` detects dupes, returns empty when none
- [ ] Duplicate repair keeps first, reassigns rest

## Verify

- [ ] `cargo test -p tomb-cli -- id` — all ID tests pass
