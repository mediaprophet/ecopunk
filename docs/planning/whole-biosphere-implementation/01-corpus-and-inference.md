# Corpus, source resolution, inference and user-defined estimation

28 October 2026. Proposed implementation contracts for consideration, under IP-01–03 and BSA-01/02/08–12. See [delivery order](00-critical-path.md). Existing native owners remain authoritative. No implementation or execution is claimed by this document.

## Required outcome

A person can acquire permitted species/place/environment evidence, inspect its source namespace, query and reason over it using QualiaDB, select a declared computation for missing/invalid data, and use the resulting supported constraint in an editable asset. Source observations, invalid records, estimates, logical consequences and artistic choices remain distinguishable. The complete corpus supports plants, animals, funga, microbes, Earth systems and space weather.

Follow the [existing namespace/inference contract](../tech/evidence-led-asset-production.md#7-source-namespaces-and-native-symbolic-inference). Reuse the Australian and Earth/space registers, existing acquisition inbox, native graph/lexicon, parsers, logic, statistics, authority and package mechanisms. External JSON/CSV is an input format; it does not become a replacement application store.

## C1 — Source-preserving ingestion

**Requirement and seam.** Native CSV and JSON acquisition already delegates from `qualia-cli/src/ingest/{csv_mapper,json_mapper}.rs` to Core `sparql_library/parsers/{csv_parser,json_parser}.rs`. The inspected parser bodies allocate random subject hashes for records, use context zero, and turn selected failed numeric conversions into zero. CSV strings are hashed on this path. This specific path does not establish the stable identifiers, invalid-value preservation or reversible lexical associations required here. Other ingestion paths must be checked before extending these interfaces; this is not a finding that all QualiaDB ingestion is lossy.

**Implementation packet.** Select an existing source-aware ingest route or extend the owning native mapping profile compatibly. Carry provider/dataset/schema/namespace, stable key rules, observation context, field type/unit and lexical provenance into each emitted assertion. Keep original bytes or permitted references and source records. Mint explicitly local source-qualified identifiers when the publisher supplies none. Resolve namespace terms through existing native lexicon owners; collisions or ambiguous mappings produce diagnostics. Keep current legacy behavior explicitly versioned where compatibility requires it; the qualified production route must reject or quarantine invalid values rather than silently converting them to zero.

**Acceptance planned.** Same source key across repeated imports resolves consistently; different datasets with the same key remain distinct; leading-zero identifiers survive; malformed numbers, missing, null, empty and true zero remain distinguishable; units and language/datatype survive Q42 save/reopen; invalid records retain a reason and original value. Snapshot-local records without durable keys must not acquire a fabricated organism identity. Test bounded input/record/field sizes and cancellation through the selected existing job owner.

## C2 — Source alignment and inference closure

**Implementation packet.** Bind qualified mappings between source vocabularies and appropriate domain vocabularies to a versioned corpus revision. Include temporal validity, taxon concept versus name versus specimen, measurement method and geographic/state scope. Select the existing native rule or solver operation for the question; retain its actual semantics and limits. Drive it through the shared Vibe/Poet host operation. Persist the returned conclusion and available evidence using existing graph/provenance owners, with links to premises, mappings, rules and engine version. Never fabricate a proof from an aggregate match count.

**Existing trace.** The pinned [prior read-only trace](../tech/evidence-led-asset-production.md#74-bounded-source-reconciliation-and-required-qualification) covers Core N3 RuleEngine, native `graph/n3.rs`, ASP owner/host, Vibe LocalHost and browser request construction. The inspected native N3 route reports matches and optional evidence; it is not itself proof of a materialized general closure. Existing ASP source-program and legacy context-enumeration paths differ. Reconcile additional native inference owners before selecting or extending the consumer. Source tests are not executed qualification.

**Acceptance planned.** Known premise/rule/query cases produce independently expected answers; a same-label cross-source mismatch is rejected; contradictions/unknowns remain visible; budget exhaustion is distinct from no answer; updating or withdrawing a premise invalidates affected conclusions and model decisions. Changes to an inference must not silently rewrite a source observation.

## C3 — Declared, user-defined computation for invalid or missing values

**User direction.** When a value is invalid, use a graph function to estimate it from other collected comparable values where appropriate. Declare the method so the person can select or define it, including using information they are authorized to access privately. Median is one available method, not a mandatory default for every quantity.

The computation is a versioned, graph-described method executed through existing native operations and Vibe source. A method declaration binds:

| Field | Required meaning |
|---|---|
| Method identifier/revision and author | Identifies the selected computation and who defined/adopted it; preserve earlier revisions. |
| Target quantity and triggering state | Property, expected units/type, referent/context, and whether invalid, missing or contested values are eligible. A contested valid observation is not automatically discarded. |
| Input selection | Versioned graph query/cohort, source permissions, independence/deduplication rules, quality exclusions, time/locality/taxon/development/measurement comparability. |
| Computation | Native operation identifier or inspectable Vibe method source; parameters, transformations, unit normalization, weighting, minimum evidence and uncertainty/error treatment. |
| Failure behavior | Insufficient data, incompatible units, nonfinite results, unsupported method, denied input, budget exhaustion and cancellation each produce an explicit unresolved/failed outcome. No zero sentinel. |
| Adoption and use | User selection or previously adopted project policy; permitted uses in preview, simulation, production or review; whether automatic estimates require review before consequential edits. |
| Result lineage | Original invalid assertion, admissible input revision identifiers, exclusions, method version, execution status, estimate and uncertainty, downstream constraints affected. |

**Concrete workflow.** Select the invalid property in Poet → inspect the original source and validation reason → choose or edit the method and comparable-input query → preview cohort coverage and exclusions → execute in the authorized native context → inspect estimate/range and derivation → adopt for the declared use. A user may establish this policy once for repeated operations; do not prompt on every value. Altering the policy or supplying better evidence creates a new result revision and invalidates dependent results as appropriate.

**Example, illustrative only.** A leaf-width record contains nonnumeric text. A user chooses a median over independent valid measurements with the same taxon concept, organ, developmental state and compatible method, converted to metres. The selected native operation computes from that bounded cohort. The graph retains the source's invalid text and adds a separate estimate with the method and inputs. If no adequate cohort exists it stays unresolved. The same method cannot be assumed valid for categorical traits, spatial coordinates, extreme events or a space-weather forecast. Appropriate interpolation, weighted estimates, models or explicit authored assumptions require their own declared applicability.

**Native reuse.** Existing `solvers/statistics/descriptive.rs` exposes `median_sorted`/`median_in_place`; `poet_host/invoke/stats/descriptive_more.rs` exposes the host median operation; Vibe catalogue includes `Statistics.median`. `specialized_libs/statistical_computing/library.rs` also has a median API. Reconcile these actual callers and their finite-value, empty-input, precision, scratch and result semantics before choosing the production route; do not implement another median kernel simply to support this workflow. The required integration is graph cohort selection, method declaration, execution context and lineage around the existing operation.

## C4 — Private, proprietary and classified inputs

The user can define methods that use sources they are authorized to access. Record the method and execute under existing project/session/source permissions. No request to upload confidential data or to disclose a private method is needed to define this contract. User authority over the project does not itself establish rights to every external dataset; use the actual source permissions.

Permission applies to source records, method code, query results, input counts, explanations, logs, caches, derived estimates and exported packages. A median or other aggregate may reveal restricted information; aggregation does not automatically grant distribution rights. Evaluate the result's release policy using existing native rights/authority mechanisms. Public viewers can receive an authorized summary or explicit unavailable status while detailed reproducibility remains within the permitted environment. Do not leak protected identifiers or values in diagnostics, tool receipts or screenshots.

Use local/authorized execution for restricted inputs, preserve provenance at the allowed granularity, and require an explicit authorized release decision where applicable. Record changed/revoked access and recomputation implications without falsely declassifying or erasing history. This plan specifies access-aware behavior; it does not certify a classified-computing deployment or implement a new classification service.

**Acceptance planned.** Authorized users can select/define a method and use permitted private inputs; unauthorized users cannot query inputs or recover them via explanation/errors/cache; a public derivative cannot inherit unrestricted disclosure merely because its numeric estimate exists; policy/source/method changes invalidate affected approvals. The principal can inspect and override adopted methods within their authority, with attributable history.

## C5 — Corpus-to-asset packaging and change propagation

Reuse Q42 graph/lexicon and HMC resource inventories, canonical M64/10d payload owners and existing source/review history. Keep dense samples/media in their typed resources. The corpus binds sources, methods, logical rules, conclusions and editable parameter decisions without expanding every pixel or sample into a graph assertion. Package the permitted full release inventory and identify unavailable/restricted resources; a bounded resident view is not the full corpus.

The existing [Q42/HMC streaming review](../../qa/implementation-2026-10-26/biosphere-integration-review/graph-streaming-reuse.md) traces range queries, lexicons, typed bindings, HMC plans, rights and residency. Its findings apply to its pinned snapshot; recheck current source before allocating an unresolved integration task. Start qualification with admitted local resources and extend remote partial acquisition only under the existing integrity contracts.

**Acceptance planned.** Corpus, method, source and parameter bindings survive fresh reopen; a changed measurement or estimation method identifies affected asset parts/reviews; a revoked source cannot appear in an unauthorized derivative; exact package resources are used by the actual consumer. Source presence, software checks and human acceptance remain separate.

## Ownership, dependencies and handoff

C1 precedes qualified corpus use; C2 and C3 share C1's source/identifier context and may progress separately. C4 constrains all operations from the start. C5 binds their output to the production path. Coordinator owns shared profile/API/dispatch edits; workers own disjoint source adapters, method UX or fixtures only after current implementation is reconciled. Preserve compatibility and source preimages. Meaningful regression tests are planned above and run serially only in the authorized qualification phase. No executable example, invented species measurement, private source or complete model is produced by this document.

Source inspection for this revision covers the CLI mapper bodies, Core parser prefixes through the field conversion branches, and selected statistics owner/host bodies. It is not a complete parser/inference/security audit. The earlier graph/logic review is cited at its original scope rather than presented as a fresh execution. Source pins are recorded in the documentation delivery receipt.
