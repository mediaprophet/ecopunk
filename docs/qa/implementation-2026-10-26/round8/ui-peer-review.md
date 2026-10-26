# Round 8 UI completion preservation — independent review

2026-10-26. Reviewer: edit-safety worker. **No blocking finding in the inspected
native product path.** This is static review only; no Cargo, test, launch,
Computer Use or human usability evidence was produced. Candidate files were not
edited by the reviewer.

Reviewed `tools/poet-foundation/round8/ui/manifest.json` and all three before/after
images. Before and after hashes matched the manifest at the end of review:

| Poet target | Reviewed after SHA-256 |
|---|---|
| `native/src/graphical/draft_buffer.rs` | `ee26aad1cc74388f0ffa9de0adbd07b95d4da1f12529b29ac0e6c92a1d92c749` |
| `native/src/graphical/ui.rs` | `58db834787d3b509bbbd88c99f4d8c9d4c428486ee814beff467bf1ad28017c6` |
| `native/src/graphical/authoring/controls.rs` | `2e3dd298191567e13109be590b347838bdd93085298c2c421dfd71c8442c0e6a` |

The final manifest SHA-256 is
`b7553dcef27bf74236cd658c0541c81929704d1eebf473312695605b66882b96`.
The supervisor reported these candidates saved with preimage guards; this worker
did not apply sibling changes. The final buffer-identity hardening was read and
reviewed separately after the initial three-file review.

The canonical native entry passes its per-controller `DraftBuffer` into
`NativeStudio`; source typing, explicit accepted-source adoption and staged
material drafts update that same buffer. Captures now retain its revision as
well as text. Successful completions require unchanged visible text and exact
buffer revision/text before replacing either. A local edit followed by restoring
the original string changes the revision and prevents late adoption. Revision
exhaustion holds automatic adoption while `set` continues retaining explicit
local typing. Refusals do not update text or increment the buffer revision.

Both completion routes first use the existing source/session/contributor
freshness check. Their current reply branches call the adoption helper with
`operation_succeeded`; a guard refusal may return a snapshot but cannot replace
the draft. Signal replacement follows the mutex comparison synchronously with
no intervening await. Native UI input handlers therefore cannot interleave in
that synchronous task. A changed buffer from another owner is checked under the
mutex; the design does not claim a general transaction over arbitrary externally
mutated signals.

Preview handling also improves the refusal path: queueing no longer clears an
accepted frame, and unchanged current refusals retain it. Changed source,
execution epoch or held reconciliation clears the old frame or installs the
returned frame. A successful authoring revision still refreshes the current view
and inspection, while retaining newer private text with an explicit status.

The parameter help text correctly describes the conservative baseline guard,
refusal after later edits, explicit source merge/separate derivation and the
unqualified UV/detail remap. It no longer advertises overwriting overrides as
normal behavior.

One scope limit should remain explicit:

* The optional `buffer=None` diagnostic path compares visible text only, so it
  cannot distinguish edit-then-undo ABA. The inspected native product path supplies
  a buffer. A future caller requiring the same guarantee must supply one or add
  local revision tracking.

The final `DraftCapture` retains its original buffer and uses the existing
pointer-identity equality before adoption. A different controller buffer with the
same text/revision now refuses, closing the initial generalization concern. The
additional staged case explicitly exercises cross-buffer refusal.

The new helper cases cover unchanged success/refusal, edit-then-undo, visible
text mismatch and optional-buffer mismatch. They are staged, unrun cases. Native
async compilation and actual contributor/reviewer interaction remain supervisor
verification and human acceptance work. This review does not close FND-05,
IP-03 C1–C8 or the full nondestructive authoring requirement.
