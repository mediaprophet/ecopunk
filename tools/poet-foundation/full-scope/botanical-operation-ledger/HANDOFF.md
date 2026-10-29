# Staged operation scope, not production admission

This folder holds a non-cloneable ledger and staged copies of current Poet
controller, host, validation, execution and module files. It is not installed,
compiled or tested. Compare to the canonical sources, reconcile later edits,
and obtain independent review before any promotion. No Cargo checks were run.

The staged edit path carries one instance from before source generation into
submit and botanical validation. Ordinary submit creates one instance; rebuild
keeps one through replay and final validation; separate inspection gets one.
Legacy v1/v2 generation and existing limits remain unchanged. The current v3
`poet_botanical` validation is explicitly refused, and the existing recipe
decoder already rejects its unsupported version/schema. No new junction
binding or Vibe operator is registered here.

The ledger counts cumulative reserved allocation bytes with no refunds, which
is conservative for peak accounted allocations but can over-refuse. Its
reservation inputs must come from owner-checked arithmetic, not authored cost
claims. In particular, these lifecycle copies DO NOT yet charge candidate
generation, validation, Vibe work, replay, retained values, source cloning,
serialization or any allocator activity. They therefore are not a resource
guard. Existing Vibe submission and replay counters are separate, and
`botanical_operation()` currently selects submission caps even during rebuild;
the real integration must use the rebuild's explicit total work and dispatch
policy while sharing or conservatively combining all work counters.

The post-submit botanical validation runs after effects may have been
attempted. This is not a pre-execution admission gate, nor does checking a
version in a post-execution value protect an independently registered fork
operator or a future different binding. Do not enable either until every entry
point rejects before expensive work/effects and complete source, operation and
memory budgets are enforced. Failed operations retain charges and outputs stay
private; accepted source and reconciliation semantics must remain intact.

The staged typed edit entry rejects `SetRecipe` versions other than 1 or 2
before `edit_source` generates or serializes anything. The raw-source submit
path now checks the existing session byte/revision limits before cloning its
optional source-layer snapshots. This is an early enforcement of already
existing limits, not a new source language or a complete allocation budget.
`SourceSession::prepare` also bounds source bytes before parsing. Source strings
can compute dynamic values and invoke effects, so text scanning cannot reliably
identify v3, and `submit_validated` validates only after evaluation. Generic
submit and replay therefore have no proven pre-effect v3 guard here. The safe
status is to leave the fork schema/operator unregistered; solving generic
admission requires an actual pre-effect checked-program restriction or an
effect-free transactional preflight in the existing Vibe host/session owner,
and a review of semantic compatibility. The staged codec's bounded graph
preflight is separate and does not bound the entire Vibe value or its clone.

Independent review identified that the source-layer snapshot allowance still
uses `max(retained_value_bytes, 64 MiB)`, potentially exceeding the configured
retained-value limit. This early byte check does not bound that snapshot, the
runtime values, parser or clones. Any tighter snapshot cap requires a separate
legacy compatibility assessment. The retained-cell sum is itself not charged
to the staged ledger.

The distinct sampled-field and topology accounting proposals are in the
adjacent `botanical-budget-owner-bounds` folder. They are incomplete structural
counts, not measurements or all-resource bounds. Original implementation
copies came from `C:/github/poet/native/src/graphical`; file comparisons are
required because another process may have modified shared files since capture.
