# Bounded Windows checkpoint persist correction

The C5 full-suite run reported a single Windows `Access is denied` (OS error 5)
while replacing a checkpoint in `native_save_conflicts_invalid_bytes_and_budget_preserve_destination`.
The retained C5 exact-test reproduction passed; the supervisor additionally
reported 100 exact-test repetitions passing. Inspection found no retained local
destination handle: bounded reads close before returning, SaveReceipt retains no
file handle, and the installed file from an earlier save closes on return.
The OS-level cause remains unknown. No antivirus, indexer or other external actor
is identified by this evidence.

The saved correction in `studio_checkpoint/native_io.rs` retains the same
synced NamedTempFile and the caller's writer lock while retrying only Windows
persist errors 5, 32 or 33. Retry pauses are 5/10/20/40/80/160 ms under a 315 ms
retry-admission/wait deadline; synchronous filesystem IO is not forcibly timed
out. Every attempt rechecks ExpectedFile and the destination. All other errors
return immediately. Exhaustion returns the first and last persist errors,
destination/staged paths and attempt count without claiming success. There is no
delete, truncate, unlock or destructive replacement fallback.

Three Windows tests in the existing checkpoint tests file exercise actual native
deny-delete handles. A private failure observer releases a handle only after a
real failed OS persist, avoiding sleep-based test races or global hooks. Tests
cover successful bounded recovery, permanent refusal preserving destination
bytes and cleaning staged files, and changed expected bytes refusing the next
replacement while preserving another writer's contents. The observer is a no-op
in production. Existing HMC validation and conflict tests remain intact.

`checkpoint-persist-before.json` and `checkpoint-persist-after.json` pin the
two-file source transition. Rustfmt passed. The worker ran no Cargo commands;
executed qualification belongs to the supervising rerun receipts. This source
correction does not retroactively turn the failed C5 full suite into a pass or
establish a root cause for its intermittent OS refusal.
