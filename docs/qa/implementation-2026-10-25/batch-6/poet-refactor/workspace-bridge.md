# Existing Poet model and native intent projection

The saved `native/src/graphical/poet_workspace` module directly includes the
actual existing portable Tool-Chest core, bundled construct catalogue and Studio
manifold seed by source path. These original sources and their Poet copyright
and licence notices are unchanged. The MIT selection for imported geometry
helpers does not relicense this Poet model or the entire application.

The projection retains the original bundled `poet` observer construct and its
full lens identifier list, rather than making a source record into a new
construct. Studio retains Dual Studio, Scene session and Audio session
containers, with an added source/part inspector projection. Native qualification
is labelled partial; unmounted Audio remains held. Original Tool-Chest,
Manifold, ContextRef, target and intent types supply the semantics.

`NativeIntentBus` implements the original generic async IntentBus trait over the
same native jobs HostHandle and opaque Context. A strict bounded in-process
parameter envelope maps known source/authoring commands to existing jobs;
there is no second worker, persistent model, journal or authority roster. This
envelope projection is not a new persisted format or external CBOR-LD transport.
Foreign schema contexts/targets, supplied actor provenance, unknown commands,
classification mismatches and oversized inputs are rejected.

Before an ordinary command, an actual authorized Workflow Inspect obtains its
admitted principal subject under the same source/session context. The legacy
`Provenance.emitter_did` field carries that qualified subject unchanged; no DID
prefix or current_user fallback is generated. The worker repeats current native
admission at execution. Receipt Accepted requires actual `operation_succeeded`
and a current result; current backend-failure replies remain available for
diagnosis, while stale source replies are withheld. This is command execution
status, never human acceptance, model acceptance or proof of settlement. Intent
counters are process-local metadata, not durable/out-of-process uniqueness or
cryptographic authority.

Refresh preserves the existing same-actor recovery path: a read-only native
refresh may discover the current source successor only when old/new opaque
authorization stamps compare equal and remain current. Workflow inspection then
proves the actor against that refreshed context before publishing it. There is
no actor replacement fallback. Cancellation is explicitly rejected as
unsupported because this bridge owns no cancellation registry/mechanism.

Five focused test sources cover original model/lens retention, real signed
software-agent NativeAuthority/NativeProject dispatch and provenance, current
backend errors, forged/foreign/unknown/oversized envelopes, revoked sessions,
unsupported cancellation and same-actor refresh after source submission. The
test executor uses safe standard-library futures/thread wakeups. No test calls
the OS vault, opens a GUI or claims GPU pixels. Cargo execution belongs to the
supervisor; no pass is inferred from saved source or rustfmt.
