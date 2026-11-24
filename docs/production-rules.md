# Production rules (biosphere encyclopedia)

Machine file: [`production-rules.json`](production-rules.json). This is Ecopunk policy, not a new morphism and not a new encyclopedia repo.

## Biosphere / non-fiction

A living taxon (flora, fauna, fungi, habitat) is **non-fiction**. Before modelling geometry or assembling a species corpus:

1. Read the datasource registers under `docs/planning/data-sources/`.
2. Fetch or bind authority sources (agent `web_fetch`, then local files under `SYNTH_INPUT_ROOT`).
3. Seal Turtle/N3 to `.q42` with Synth MCP `synth_compile_q42`.
4. Inspect with `query_sparql` (`vault_path` on that `.q42`).
5. Then produce (`compile_botanical_organ_10d`, `fspm_vascular_grow`, `qualia.species.assemble_corpus`).

Fiction, studio chrome, and anatomy follow their own repos/rules. `compile_q42` and graph queries are the evidence path; they are not blocked.

Synth evaluates these rules when `synth_produce_present` is called with `domain=biosphere` (and `fiction` is not true), or when an agent calls `synth_production_rules`.
