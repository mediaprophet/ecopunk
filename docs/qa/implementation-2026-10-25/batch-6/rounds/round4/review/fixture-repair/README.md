# Public predecessor fixture isolation repair

Test-only stage after the actual public run reported 11 passed, one failed.
No production policy change is proposed. No build or test was run by this worker.
The original failure/log remains in `round4/public.log`.

The failed assertion required `replace_admission` to report the predecessor
contributor denial. Its candidate new grant changed only `id`, while retaining
the existing grant's exact target and reviewer. `SignedAdmission::verify` invokes
`validate_admission_packages` before predecessor policy; that earlier duplicate
scope predicate rejects the body with `invalid/duplicate exact package-review
admission scope`. This is a source-backed diagnosis of the earlier branch;
the failing run did not print the returned string.

The repair assigns the new grant a second declared project-scoped release alias
for the same saved bytes, source and producer. The original signed grant remains
unchanged and the sequence advances once. Added assertions pin those facts.
An isolated fresh native owner must admit the signed replacement body before
the original history-bearing owner is asked to replace admission. That positive
control proves signature/project/subject/grant shape is valid, leaving the
original owner's recorded predecessor contribution as the intended denial.
The final assertion retains the `predecessor` requirement and now prints the
actual returned error if a different branch fails.

`target-manifest.json` captures the exact currently applied fixture before-image
and candidate hash. Apply only after the supervisor's running producer command
has exited and its source pin has been recorded. No live mutation occurred here.
