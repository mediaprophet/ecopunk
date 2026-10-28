# Initial rig adoption — staged candidate

Purpose: let the admitted operator bind a selected existing SKL1 HMC resource to a selected existing assembly part. This is a synthetic engineering scope, not biological acceptance.

The candidate extends the existing RigControl intent and its cancellation-aware host route. The checkpoint owner retains ReadSource and ExecuteSource checks plus source/package/resource association. The viewport derives source-owned descriptor fields from the selected sealed resource and its exact selected compiled geometry; admit_bound_rig validates rig/player and vertex lineage. The canonical source owner refuses existing bindings and emits one Vibe cell creating a mutable poet_rig. Existing project save/history remains authoritative.

The UI requests a package filename and inspected package digest, resource key and digest, selected part and local binding identifier. It explicitly selects the first clip in resource order. Actor/project authority is never taken from those inputs. The selected viewport part and exact contiguous geometry must match; unsupported topology, stale source, denied authority, resource mismatch or cancellation refuse before submission. Cancellation can race after the final check or an already-started synchronous submit.

Scope limits: no editing of an existing binding via the Adopt action; no remesh/weight remap beyond identity, no initial clip choice beyond resource order, no animation upload, no human usability/domain acceptance. Local ReadSource does not establish a third-party right; resource terms remain separate. Whole-HMC decode and selected geometry precede existing admission budget and are not an aggregate peak-memory guarantee.

Synthetic canonical source-owner test covers creation and refusal of replacing an existing binding. It uses a fake preview provider, not retained viewport/GPU, signed project, HMC checkpoint or fresh process. A real signed project resource selection → viewport correspondence → adopt → save/reopen case plus stale/denied/cancelled/foreign cases must be implemented and executed.

manifest.json pins six preimages, candidate hashes and destinations. Rustfmt parser accepted staged sources. No Cargo, build, native window, test or model production was executed by this worker. The coordinator owns preimage-reviewed install and serial checks; after copy update file mtimes to prevent Cargo reusing stale test binaries.

Suggested focused check (coordinator only): cargo test --locked --offline -p poet-cli --features native-graphical --lib rig_ --target-dir C:/github/ecopunk/target. This cannot alone establish the operator workflow.

A test now seeds the actual retained viewport CPU scene and selection without a GPU and calls its real adoption method. It checks derived digests, joint/clip IDs, first clip, vertex lineage, and wrong part/bind, corrupt resource, changed geometry and cancellation refusals. It does not prove an uploaded GPU frame, signed authority, package concurrency or HMC save/reopen. This worker only parsed the staged test; it was not executed.
