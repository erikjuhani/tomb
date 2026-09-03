# File I/O

Implement `tomb-cli/src/io.rs` — safe file operations with locking, atomic writes, and optimistic concurrency.

Prerequisite: 004-parser.md, 005-renderer.md, 006-id-system.md

## 1. Module scaffolding

- [x] Create `tomb-cli/src/io.rs`
- [x] Add `pub mod io;` to `lib.rs`
- [x] Add `NotManaged` and `Conflict` error variants to `TombError` if needed
- [x] Verify: `cargo check -p tomb-cli`

## 2. Read — `read_managed_file(path)`

- [x] Acquire shared (read) lock via `File::lock_shared()`
- [x] Read file contents to string
- [x] Release lock (drop)
- [x] Parse with `parse_managed_file()`
- [x] Return `(ManagedFile, String)` — model and original source for conflict detection

## 3. Atomic write helper

- [x] Write to `path.with_extension("md.tmp")`
- [x] `fs::rename(tmp, path)` — atomic on POSIX
- [x] Clean up tmp file on error

## 4. Write — `write_managed_file(path, original_source, file)`

- [x] Acquire exclusive lock via `File::lock()`
- [x] Re-read current file contents
- [x] Compare current contents to `original_source` — if different, return `TombError::Conflict`
- [x] Render `file` to string
- [x] Atomic write (tmp + rename)
- [x] Release lock

## 5. Mutate — `mutate_managed_file(path, closure)`

- [x] Convenience function: read → apply closure → assign missing IDs → write
- [x] `closure: FnOnce(&mut ManagedFile)`
- [x] Handles the full read-modify-write cycle with conflict detection
- [x] Returns the mutated `ManagedFile` on success

## 6. Multi-file ID resolution

- [x] `resolve_id_across_files(prefix, config) -> Result<(PathBuf, String)>`
- [x] Read all configured files, collect all IDs with their file paths
- [x] Use `resolve_prefix` to find the target
- [x] Return the file path and full ID

## 7. Tests

- [x] Read + parse a file successfully
- [x] Write + re-read round-trips correctly
- [x] Conflict detection: modify file between read and write → error
- [x] Mutate applies closure and writes
- [x] Atomic write: original file untouched if write fails mid-way
- [x] Resolve ID across multiple files

## Verify

- [x] `cargo test -p tomb-cli -- io` — all I/O tests pass
- [x] Tests use `tempfile::TempDir` for isolation
