# Native authority publication stamp follow-up

**Staged, unexecuted and not applied by this worker.** This guarded three-file
follow-up supersedes the pending [protocol test follow-up](../followup/apply-manifest.json)
by including those tests along with the authority publication change. Original
gateway and prior follow-up source/receipts remain separate. Parent owns snapshot,
worker context, UI and MCP publication checks and the coherent actual build.

`AuthorityOwner::publication_is_current(generation) -> bool` is a nonblocking
native owner check. Default is false. Providers must return false on contention,
revocation, expiry, disconnection or generation mismatch and cover every relevant
authentication/session/membership/review change. The gateway cannot enforce a
provider's implementation of nonblocking behavior; that is a native provider
qualification requirement. This method creates no permit or authentication.

`AuthorisationStamp` keeps an opaque native owner Arc and admitted generation.
It is Clone/Debug/Eq; equality uses owner Arc identity plus generation. It cannot
be serialized/deserialized or constructed from source, MCP arguments or a saved
checkpoint. `is_current()` checks the provider's fast guard. A newly admitted
generation cannot revive an older completion stamp.

`AuthorizedEditorHost::snapshot` attaches a stamp using the generation captured
inside the existing authorized owner callback. It does not issue a second owner
transaction. Publication consumers must recheck this stamp alongside the actual
source StoreStamp and execution epoch immediately before installing/returning
source, frame or metadata. Root controller snapshots may retain None for an
explicit diagnostic profile; that is not cooperative authorization evidence.
All source-bearing write outcomes (submit/material, undo/redo and rebuild) receive
the same owner/generation stamp inside their admission callback, matching the
final wrapped snapshot when native authority has not changed. Save receipts have
no embedded source snapshot and retain their existing representation. Worker/MCP
integration must recheck the captured stamp before publication. A provider
without a fast guard may still execute authorized native operations; its delayed
authorized data cannot be published as current.

Two new fixture tests cover revocation after a completed snapshot, generation
change, contention returning false without blocking, and default-false providers
retaining operation admission without granting publication. The two pending
protocol/native source tests are included unchanged. No real human credentials,
window control, fake GPU frame, production provider or human acceptance is
claimed. Rustfmt passed; all three files remain below 500 lines. Tests remain
unrun pending the parent's coherent current-source dependencies and snapshot
field integration. IP-03 C1–C8 and complete film/game/cartoon/HMC qualification
remain open.
