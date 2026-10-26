# Native unknown-completion preview correction

Status: staged, not applied and not runtime-tested by this worker. Two exact
current-live before images and proposed after images are pinned by
`manifest.json`. No sibling source writes or Cargo/build/test commands ran.
Both staged files pass rustfmt check. Git no-index diff returned its expected
exit 1 because the candidates differ; the changes are only the two error
branches and the authoring stale-completion branch.

The risk is real. In `jobs.rs`, the worker invalidates its completion cache,
then calls `execute(host, ...)` before refreshing the cache and sending the
oneshot result. Host work may commit before a later panic, lost sender or reply
failure. `jobs::run` maps a failed `receiver.await` to a String error explicitly
warning that started effects may require reconciliation. The native intent bus
propagates that error; `native_dispatch::run` returns it to both UI dispatchers.
A generic Err therefore cannot prove source was unchanged.

The candidate clears only the derived image URI on generic Err in `ui.rs` and
`authoring/controls.rs`. Authoring controls also clear the URI when completion
revalidation fails, matching the existing main UI policy. Draft text, accepted
source view, error message and retry behavior are unchanged.

A normal operation or guard refusal that returns a current `Ok(reply)` with
`operation_succeeded == false` and unchanged source/session still follows the
existing guarded reply path and retains its valid previous preview. There is
no clear-on-queue behavior and no automatic command retry. A fresh frame can
only be installed through the existing completion/current-source checks.

Root owns independent review, guarded application after its active suite and
subsequent actual-source verification. This is a bounded failure-display fix;
it does not qualify native human usability, source authority, model output or
production readiness.
