# Native HMC resource-preserving editing — saved implementation

This bounded implementation extends the existing native exact-source/private
journal checkpoint path. It does not publish a release, run a payment, grant a
capability, or establish an accepted embodiment or usable graphical experience.
All film, open-world game and cartoon model/package completion remains open.

## Saved behavior

- QualiaDB `studio_checkpoint::Checkpoint` retains every non-source HMC entry:
  exact key, kind, payload and opaque metadata. Source-entry metadata is also
  retained. Canonical `BundleReader` and `BundleWriter` perform container IO;
  native Q42 resources retain their original graph/lexicon bytes.
- Native Poet carries those resources across activation, source/material edits,
  same-file saves and Save As. A different destination/retained digest is refused
  rather than merging unrelated bases. Parent lineage binds the exact base HMC
  SHA-256, independently of identifiers or the process-local source stamp.
- Explicit Add, Replace, ReplaceBytes and Remove requests guard the selected
  existing package SHA; replacement/removal also guards the old payload SHA.
  ReplaceBytes retains exact existing kind and opaque metadata. Source/journal
  remains reserved to its existing stamped owner. Incidental compiled-output key
  collisions are refused rather than replacing retained resources.
- Preflight checks package digest, source association, current source stamp,
  native write policy and retained base before the host reads an import file.
  Final save repeats its guards. Native authority remains the surrounding owner.
- Trusted host policy explicitly distinguishes a WorkingCopy from an
  ImmutableRelease pinned by filename and SHA. The origin cannot be overwritten;
  a new working copy preserves resources and exact origin lineage. Incoming HMC
  contents do not select this policy or establish publication/review status.
- Explicit resource edits check known native `.10d` material texture dependencies
  using canonical native codecs. A missing referenced texture or a changed
  payload beneath an old content-addressed key is refused. Existing generated
  `.10d`/Q42 companion pairs must remain together or be explicitly removed
  together; changed compiled geometry requires an explicit old-digest-guarded
  companion graph replacement in the same plan. That graph's validated bytes
  may remain identical; no gratuitous knowledge change is required.
  Unknown semantic references and matching geometry/graph meaning remain
  unassessed. This is not a complete-HMC semantic validation claim.
- Native IO validates before mutation, writes and syncs a same-directory RAII
  staging file, repeats the expected SHA guard under its cooperating-writer OS
  lock, then installs atomically. After install, Unix directory-sync failure is
  an explicit successful-save durability warning, never an apparent rollback.
  Windows power-loss directory durability remains unqualified. Arbitrary outside
  writers must honor the documented lock: portable compare-SHA-and-rename cannot
  atomically exclude an unrelated writer that ignores it.
- Caller-selected file/source/entry/cold-work budgets fail closed. The default
  file budget is a configurable working-context bound, not a quality ceiling or
  arbitrary reduction of editable master scope. Unknown resources stay inert;
  reopening never executes them or restores grants/principals/effects.

## Evidence and qualification

[source-before.json](source-before.json), [source-after.json](source-after.json)
and [source-transitions.json](source-transitions.json) record observed SHA-256
source transitions. The original checkpoint router hash was observed before the
first import-only patch; that observation supplies its baseline hash. All owned
implementation/test modules remain below 500 lines.

[checks.json](checks.json) records this sub-agent's executed rustfmt check.
Cargo tests/builds are root-coordinated and were not run by this sub-agent; written
assertions below are not reported as executed passes here.

The saved tests cover actual native compiled `.10d` and Q42 resources mixed with
opaque editable source/rig/texture/unknown entries, payload and metadata hashes,
source/material edits, explicit resource changes, package/entry stale guards,
budget/corruption/schema failures, native dependency invalidation, immutable
origin derivation, source-only backward compatibility and inert resource text.
The native fresh-process fixture exercises retained mixed resources, held source
activation, explicit rebuild, material edits and Save As; its observations are
diagnostic fixtures, not high-grade master or human-review evidence. A Unix-only
symlink test is present; Windows link/junction negative execution is not claimed.

Source review of the native dependency refusal addressed reviewer finding
`B4-R04` within its stated known-native scope. Actual Cargo verification,
authenticated contributor/reviewer graphical acceptance, complete embodiment
claims and all applicable production-packet acceptance remain separate work.

## Atomic operator-plan follow-up

Native Poet now exposes one bounded `LibraryEdit` transaction through
`EditorHost::apply_resource_plan`, the native `PackageBatchControls` reviewed JSON
command draft, and MCP `poet_hmc_edit_resources`. The strict typed command schema
supports add, replace and remove, with at most 64 unique resource keys. Replacing
or removing coupled `.10d` and Q42 resources is possible in one guarded plan.
Every logical intent, source stamp, package SHA, old resource SHA and immutable
policy is checked before any selected library file is read. One host-configured
import byte budget covers the whole plan; failure during import or native
dependency checking performs no HMC install. Remove-only plans also work without
an import library. Incoming filenames never grant arbitrary filesystem access.

Focused saved native-host tests exercise actual compiled `.10d`/Q42 replacement,
stale-plan rejection, paired removal, unchanged opaque master metadata, aggregate
budget exhaustion, immutable preflight and strict UI/MCP command parsing.
An explicit companion edit with identical validated graph bytes is also covered
in the canonical core tests. These are written tests, pending the root's actual
Cargo qualification. [atomic-plan-source-after.json](atomic-plan-source-after.json)
records the saved operator-plan source hashes. No GUI or human acceptance is
claimed by the command-backend tests.
