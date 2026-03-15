# File I/O

Implement `tomb-cli/src/io.rs` — safe file operations with locking, atomic writes, and optimistic concurrency.

Prerequisite: 004-parser.md, 005-renderer.md, 006-id-system.md

## 1. Read — `read_managed_file(path)`

- [ ] Acquire shared (read) lock via `File::lock_shared()`
- [ ] Read file contents to string
- [ ] Release lock (drop)
- [ ] Parse with `parse_managed_file()`
- [ ] Return `(ManagedFile, String)` — model and original source for conflict detection

## 2. Atomic write helper

- [ ] Write to `path.with_extension("md.tmp")`
- [ ] `fs::rename(tmp, path)` — atomic on POSIX
- [ ] Clean up tmp file on error

## 3. Write — `write_managed_file(path, original_source, file)`

- [ ] Acquire exclusive lock via `File::lock()`
- [ ] Re-read current file contents
- [ ] Compare current contents to `original_source` — if different, return `TombError::Conflict`
- [ ] Render `file` to string
- [ ] Atomic write (tmp + rename)
- [ ] Release lock

## 4. Mutate — `mutate_managed_file(path, closure)`

- [ ] Convenience function: read → apply closure → assign missing IDs → write
- [ ] `closure: FnOnce(&mut ManagedFile)`
- [ ] Handles the full read-modify-write cycle with conflict detection
- [ ] Returns the mutated `ManagedFile` on success

## 5. Multi-file ID resolution

- [ ] `resolve_id_across_files(prefix, config) -> Result<(PathBuf, String)>`
- [ ] Read all configured files, collect all IDs with their file paths
- [ ] Use `resolve_prefix` to find the target
- [ ] Return the file path and full ID

## 6. Tests

- [ ] Read + parse a file successfully
- [ ] Write + re-read round-trips correctly
- [ ] Conflict detection: modify file between read and write → error
- [ ] Mutate applies closure and writes
- [ ] Atomic write: original file untouched if write fails mid-way
- [ ] Resolve ID across multiple files

## Verify

- [ ] `cargo test -p tomb-cli -- io` — all I/O tests pass
- [ ] Tests use `tempfile::TempDir` for isolation
