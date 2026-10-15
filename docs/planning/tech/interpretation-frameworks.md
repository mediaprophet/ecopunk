# Interpretation frameworks — data × expressed logic × instruments

**Status:** draft v0.1 (concept, recorded 2026-10-12)
**Feeds:** [statistics.md](statistics.md), [../sociosphere.md](../sociosphere.md),
[ontology-stack.md](ontology-stack.md), [review-workflow.md](review-workflow.md),
[compensation.md](compensation.md)
**Upstream substrate:** `qualiaDB/core-ontologies/semantic-instruments.n3`,
`semantic-instrument-shapes.n3`, `cml.n3`,
`docs/work-in-progress/SEMANTIC_INSTRUMENT_*.md` (SI-01 landed; runtime
WIP — see §8 status honesty)

---

## 1. The claim being modelled

Datasets do not interpret themselves. The same census table, occurrence slice,
or ecosystem account yields different results under different analytical frames
— and every reading is produced by *something*: a rule set, a statistical model,
a legal instrument, a knowledge system, a methodology document. The framework's
job is to make that "something" **first-class, addressable, and plural**:

- **expressed logic** — N3 rule sets, SHACL profiles, deontic/epistemic/temporal
  modality sub-graphs, VibeScript behaviour, WASM kernels;
- **modelling** — statistical models (PPP, MRIO, projections), classifiers,
  aggregation/concordance procedures;
- **semantic instruments** — the upstream `si:` package type, which is the
  collectable, versioned, attestable carrier for exactly this material;
- **axiom families** — SOC/STAT/DOM/COMP packages are *themselves* expressed
  logic — an interpretation declares which axiom set it runs under.

An interpretation is therefore a **bound execution**: `data slice × instrument
release × entry point × inputs → result claims + receipt`. An informal human
reading can also be recorded — as a *claim without execution receipt*, the
weaker provenance class — but it is still typed, attributed, and contestable.

## 2. Verified upstream substrate (SI-01, inspected 2026-10-12)

| Surface | File | What exists |
|---|---|---|
| Instrument ontology | `core-ontologies/semantic-instruments.n3` | `si:SemanticInstrument` (abstract, version-independent), `si:InstrumentRelease` (immutable), `si:InstrumentPackage`, `si:EntryPoint` (named/typed/bounded dispatch), `si:DependencyRequirement` (ontology/lexicon/law-package/dataset/native-module/**instrument**/sector-profile kinds), `si:HonestyPolicy` (fail-closed), `si:Applicability` (jurisdiction/population/period/units), `si:ResourceLimits`, `si:TestManifest`, `si:Citation`, `si:ExecutionEvent`/`si:ExecutionReceipt`/`si:ResultClaim` |
| Lifecycle | same | Five independent state axes — authoring, publication (`Unpublished/Published/Withdrawn/Superseded/Revoked`), resolution, installation, trust (`Unassessed→Endorsed/LocallyTrusted`), activation; `si:contentCategory` Demo/Reference/**Operational** must stay labelled |
| Logic routing | `core-ontologies/cml.n3` | `cml:LogicApplication` + `cml:Modality` (`N3Logic`, `SHACL`, deontic, epistemic, LTL…) — **subject-matter-selected evaluator routing**; `si:EntryPoint.logicApplication` reuses it |
| Honesty flags | same | `si:WhoToolClaimCollapseFlag`, `si:SignatureAsTruthFlag`, `si:UngroundedSoftwareAgentFlag`, `si:ArtworkAsProofFlag`, etc. — category errors are *routed flags*, not silent failures |
| Shapes/fixtures | `semantic-instrument-shapes.n3`, `fixtures/semantic-instruments/` | Positive + negative fixtures incl. `negative-who-tool-claim-collapse`, `negative-revoked-presented-as-active`, `negative-missing-honesty-policy` |
| cml corpus | `core-ontologies/concepts/` + `un-instruments/` | 104 `cml:Concept` logic layers over 102 UN instruments — `cml:realizedBy` verbatim text, `cml:asserts` deontic norms, `cml:Proposed` pending attestation |

**Honest status:** the *vocabulary and contract layer* exists and is test-gated;
runtime dispatch/resolver/Poet-Webizen UI are upstream WIP (SI plan packets),
and several decisions are HELD (canonical carrier, credential consolidation,
digest algorithm, sector-profile endorsement authority). Treat `si:` as
**upstream-partial**: adoptable as the schema now; execution claims per-gap.

## 3. INT-AX — interpretation axiom family

| Axiom | Statement | Rejects |
|---|---|---|
| **INT-AX-1 Interpretation is a bound execution** | Every derived output links `{data slice, instrument release, entry point, input commitment}` via an `si:ExecutionReceipt`; a derived number without its instrument is a claim missing its method. | Anonymous derived figures; "analysis says" without a tool |
| **INT-AX-2 Instruments are plural by construction** | Rival instruments reading the same data produce coexisting claims — different schools, methodologies, traditions, jurisdictions. Contest is recorded; no silent resolution (paraconsistent modality; CONTEST machinery). | One blessed reading per dataset |
| **INT-AX-3 Results are claims, never facts** | `si:ResultClaim` carries epistemic band + applicability + receipt link; "not automatically a fact, diagnosis, or authoritative decision" (upstream comment). | Output promoted to observation-layer |
| **INT-AX-4 Applicability is enforced, not advised** | `si:Applicability` bounds (jurisdiction, population, period, units) gate execution; incomplete inputs `si:Held`/`si:FailClosed` — never defaulted to plausible-looking values. | Silent extrapolation outside domain |
| **INT-AX-5 Prohibited interpretations are declared** | `si:prohibitedInterpretation` — including the **ecological-fallacy rule** (STAT-AX-2): a population statistic may not be applied to a person; an index may not rank individuals. | Aggregate→individual reasoning |
| **INT-AX-6 Axiom sets are instruments** | SOC/STAT/DOM/COMP families publish as versioned axiom packages (SHACL + N3 rules + shapes) consumed via `si:DependencyRequirement`; an interpretation declares its axiom context — the same data under custodial vs institutional axiom sets is *two interpretations*, both recorded. | Axioms as ambient assumption |
| **INT-AX-7 Instruments compose with receipts** | Instrument→instrument dependency chains keep per-hop receipts (`si:parentReceipt`); a chain's result cites the whole resolution set (`dependencyLockDigest`). | Opaque pipelines; unversioned upstream model |
| **INT-AX-8 Sources are heterogeneous; contracts are not** | N3 rules, SHACL, modality evaluators, VibeScript, WASM kernels, statistical models, LLM-assisted extraction — all enter as `si:` releases with the same lifecycle/honesty requirements. Source kind never lowers the bar (an LLM-produced reading needs the same receipts + a principal chain). | Special-casing "AI" outputs |
| **INT-AX-9 Interpretation ≠ modification** | Reading data never mutates the observation layer; instrument outputs are *additions* (claims/derived volumes via `wasDerivedFrom`). | In-place "correction" of source data |
| **INT-AX-10 Trust state is earned and visible** | `si:TrustState` Unassessed→SelfAsserted→Reviewed→Endorsed/LocallyTrusted is rendered in interfaces; a revoked release may not present as active (negative fixture exists). | Badge-as-proof; hidden trust status |

## 4. Instrument kinds register (extensible — vocabulary additions, not schema)

| Kind | Examples for ecopunk | Execution surface |
|---|---|---|
| **Rule set** | N3 defeasible rules reading occurrence data ("if species X + habitat Y + season Z → plausible nesting record") | `cml:Modality N3Logic` — Strict/Defeasible/Defeater/Linear arrows (n3_parser verified) |
| **Shape profile** | SHACL validation reading whether a dataset *supports* an interpretation | `shacl_compiler` / `shacl/validate.rs` |
| **Modality logic** | deontic reading (may/must/must-not uses under licence+TK labels); LTL temporal rules over series | modality evaluators (verified opcode families) |
| **Statistical model** | PPP place-factor computation; MRIO footprint model; population-projection scenario; concordance mapper | `si:` release wrapping a declared model + versioned parameters |
| **Classifier / ML** | bioacoustic species classifier; relation extraction from literature | bounded WASM/native kernel dependency; results stay `Inferred`-class proposals |
| **Axiom package** | custodial-axiom reading vs institutional-axiom reading of the same custodianship data | packaged SOC/STAT/DOM/COMP axiom sets |
| **Knowledge system** | Indigenous seasonal-calendar readings of phenology data — a *governed* instrument whose authority sits with the community, executed under its declared conditions | instrument + sovereignty lane + TK labels |
| **Legal instrument** | `cml:` concepts over un-instruments; law packages as `si:LawPackageDependency` | cml logic layer |
| **Informal reading** | a human's stated interpretation without executable logic | claim record w/o receipt — weakest provenance class, still first-class |

## 5. Binding model

```
Observation layer (immutable)
      │  qb:DataSet / dwc:Occurrence / Volume slice
      ▼
si:ExecutionEvent ── employsRelease ──► si:InstrumentRelease
      │── employsEntryPoint ──► si:EntryPoint { inputShape, outputShape,
      │                                          logicApplication, limits }
      │── invokedBy ──► values:Agent (+ capacity + principal chain)
      ▼
si:ExecutionReceipt { inputCommitment, outputCommitment,
                      dependencyLockDigest, parentReceipt? }
      │
      ▼
si:ResultClaim(s) — epistemic band, applicability, contest status
```

The data stays untouched; the *interpretation* is a new graph entity with its
own lifecycle. Two instruments over the same slice produce two receipts — both
survive; a later contest or ratification (review-workflow lanes) attaches to
the **claim**, never rewrites the observation.

## 6. Plurality & the "related axioms" case

The "may or may not relate to semantic instruments" scope resolves cleanly:

- **Instrument-mediated** readings carry receipts and are recomputable.
- **Unmediated** readings (a researcher asserting "this decline tracks the
  drought series") are claims with source attribution — same namespace,
  weaker evidence class.
- **Axiom-relative** readings are first-class: interpretation I₁ = data ×
  instrument × *custodial axiom set*; I₂ = same data × instrument ×
  *institutional axiom set*. Both are recorded; the axiom context is part of
  the receipt's dependency lock. This is how SOC-AX-7's "context may stack
  multiple standing systems" gets implemented mechanically.

## 7. ecopunk application set (first instruments to spec)

| # | Instrument | Reads | Produces |
|---|---|---|---|
| I-1 | SEEA account interpreter | ABS NEA/land accounts | ecosystem extent/condition/service claims joined to `bio:`/`env:` entities (G41) |
| I-2 | ASGS concordance reader | mesh-block allocation files | edition-crossing proposals with documented loss (G40) |
| I-3 | Compensation computer | labour-class vocab + PPP/living-wage benchmarks | payable-obligation claims (COMP-AX; `royalty` inputs) |
| I-4 | Seasonal-calendar instrument | TERN/ALA phenology data | community-governed phenological-phase readings (sovereignty lane) |
| I-5 | Footprint model wrapper | consumption × TiVA/open-MRIO | embodied-impact claims (STAT-AX-9 modelled) |
| I-6 | Custodial-reading package | custodianship/Country data under custodial axioms | relation-bearing region claims — rendered with custodial grammar, not property lines |

## 8. Gap register additions

- **G44 — instrument runtime/dispatch for dataset interpretation** —
  `upstream-partial`: `si:` schema + shapes + fixtures landed (SI-01);
  entry-point dispatch, resolver, and execution-receipt generation are WIP
  packets; HELD decisions outstanding (carrier, digest algorithm).
- **G45 — axiom-package publishing convention** — `eco-spec`: package
  SOC/STAT/DOM/COMP axiom sets as `si:`-compatible release units so
  interpretations can declare axiom context as a dependency.

## 9. Open questions

1. Does a `si:InstrumentPackage` travel inside ecopunk `.hmc` packs (kind
   `"instrument"` entry) or as its own collectable HCF/HMC object per upstream?
   The decision register leans Q42-envelope carrier; packs may embed or
   reference — align with the HELD carrier decision.
2. Whether instrument results can ever feed *other instruments' inputs* inline
   (chain-of-interpretation short-form) or only via explicit chained events —
   lean: explicit events only, receipts are the chain.
3. Sector-profile endorsement authority is HELD upstream — ecopunk's
   biosphere/sociosphere sector profiles need a ratification home
   (review-workflow sovereign lane is the candidate).
4. Minimum receipt fields for the *informal* (unmediated) reading class —
   enough to be citable + contestable without fabricating execution.
5. Do pack manifests need an `interpretations` section listing which
   instrument releases produced which derived layers? Probably yes for
   reproducible packs — decide with asset-package-spec.
