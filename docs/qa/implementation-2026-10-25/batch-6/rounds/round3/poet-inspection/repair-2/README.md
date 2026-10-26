# Second test-only repair — bounded inspection invalidation fixture

Root's real six-case `--lib` run recorded three passes, two native checkpoint
save failures and one inspection-delivery case running beyond 60 seconds before
interruption. That interrupted run is not a passing suite. Root retains the
logs and owns the explicit caller-selected TEMP/TMP rerun; no persistence-engine
changes or permission weakening are proposed for the two AccessDenied results.

The new inspection-delivery fixture previously required both caller and worker
to reach an unbounded two-party `Barrier`. This is invalid when dispatch can be
refused normally. `jobs::execute` checks `origin.source.check_current()` before
calling `ProbeHost::inspect_package_resource`. Core
`studio_stamped::artifact_stamp_is_current` uses the global Studio store's
`try_lock`; a busy store returns false even if the source itself has not changed.
Other source/cooperation fixtures access the same process-global store. If the
worker returns a normal no-dispatch reply before reaching the hook, the caller
still waits forever for that hook's barrier. No timeout or reply inspection can
then run. The same failure mechanism applies to any earlier refusal/panic.

A deterministic reproduction of this failure mechanism is to retain the Studio
store mutex in an existing stamped mutation callback while the worker performs
its source-current admission. That admission must fail closed without entering
the inspector; the old caller's barrier is then unmatched. This is a
source-demonstrated sufficient mechanism, not an observed thread trace of the
interrupted run. The saved test log alone does not establish the exact timing
or barrier location of that run; no reproduction command was executed here.

The one-file candidate adds a default-false **test-only** `ProbeHost` hook. For
this inspection case only, it replaces its own actual Studio source incarnation
inside the inspector, after admission and before returning its normal typed
delivery fixture result. Existing production final-context checks must withhold
that stale result. This new case uses `fixture(None)` and no barrier. A missing
dispatch returns the normal reply to the existing bounded `receive` helper and
fails an assertion rather than waiting for a worker-only signal. Legitimate
busy/refused admission remains a failure, not implicit success or a retry.

This tests deterministic post-admission source invalidation plus the existing
session/epoch-successor branches. It does not claim a measured concurrent race.
Existing worker lifecycle/race tests remain unchanged, including canonical
production retirement, queue/cancellation and joined started work. Production
privacy, authority, job admission and persistence are untouched.

Only `native/src/graphical/jobs_tests.rs` is staged. Complete before/after source
and guards are in [target-manifest.json](target-manifest.json), SHA-256
`8e5917b4047e3625dbb84f8cc8836a2c044f44187ad465f112d00a1fc40173a0`.
Expected current hash is
`b545bbd6ee47210e534c2503889d32c63ec404e0ab2e60be1ea0dfc36aacfe51`;
candidate hash is
`785a24fdf9d5c9e1f913a580813b528bea5ca9e5fe50edede2b2fabca5d2da4d`.
No live writes, builds, Cargo/test execution, credentials, assets or Core changes
were performed by this worker. Root owns review, guarded application and actual
verification; all original attempts and earlier source snapshots remain intact.
