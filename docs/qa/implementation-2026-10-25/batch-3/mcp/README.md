# Thin native authoring MCP protocol

Staged 2026-10-25. `apply-manifest.json` pins eight targets in the existing
`qualia-mcp` crate: Cargo feature routing, lib routing, and six directory-backed
authoring module files, each below 500 lines. No sibling source was changed by
this worker. Parent owns independent review and guarded application.

## Features and public API

Default `engine-tools` preserves the existing dependency closure, modules and
public exports. Existing engine-tool implementations and their historical
protocol are not rewritten or certified by this patch. New consumers choose:

```toml
qualia-mcp = { path = ".../qualia-mcp", default-features = false, features = ["authoring"] }
```

The authoring path uses serde, serde_json and std; it calls no legacy Poet tool,
engine dispatch, listener or synthetic fallback. Module API:

```rust
trait AuthoringBackend {
    fn tools(&self) -> Vec<AuthoringTool>;
    fn call(&mut self, name: &str, args: serde_json::Value)
        -> Result<serde_json::Value, ToolError>;
}
// AuthoringTool { name: String, description: String, input_schema: Value }
// ToolError { code: String, message: String }; ToolError::new(code, message)
// AuthoringServer::new(backend, ProtocolLimits) -> Result<Self, ProtocolError>
// server.process(&str) -> Option<Value>
// server.lifecycle() -> Lifecycle; server.into_backend() -> Backend
// run_stdio(backend, limits) -> Result<(), ProtocolError>
// serve(reader: impl BufRead, writer: impl Write, backend, limits) -> Result<(), ProtocolError>
```

Backend results must be domain objects. Protocol wraps them in MCP
structuredContent and compatible text content. ToolError must carry deliberately
sanitized public code/message, never raw private diagnostics, credentials or
source text. The protocol caps these fields but cannot infer whether arbitrary
backend strings contain private data. Backend must not print to stdout.

Root owns the actual shared native Poet gateway/backend. That gateway must bind
host-held authenticated authority and recheck it per operation. JSON arguments,
MCP clientInfo, source identifiers and initialization are never authentication.
An unbound backend must fail closed; protocol success cannot supply authority.
Only operations returned by the actual backend catalog can be listed or called.
The catalog is fixed for the connection and advertises listChanged false.

## Protocol profile

The new path supports only MCP 2025-11-25. It responds with that version for an
unsupported requested version; incompatible clients should disconnect. It does
not claim to implement an arbitrary version merely because the client names it.
Initialization then the initialized notification precedes tool calls; ping is
allowed without changing lifecycle. This follows the official [lifecycle and
version negotiation contract](https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle).

Requests require JSON-RPC 2.0, a string/integer ID and a method. Null/float/bool
IDs, batches, ambiguous duplicate keys and malformed envelopes fail before
dispatch. Duplicate keys are deliberately rejected throughout nested arguments.
Valid notifications are silent and never execute tools, including tools/call
without an ID. Unknown tools, task requests and unsupported cursors fail rather
than dispatching an inferred substitute. Tool failures return isError true;
unknown methods/invalid parameters return protocol errors. Compatible object
results and tool errors follow the official [tool result
contract](https://modelcontextprotocol.io/specification/2025-11-25/server/tools).

Stdio consists solely of UTF-8 newline-delimited protocol frames; EOF shuts down.
CRLF is accepted. Overlong, invalid UTF-8 or unterminated frames close with a typed
transport error, without partially dispatching the frame. Serialization is counted
before stdout writes, and only complete bounded responses plus a newline are
written. The library emits no startup banners or diagnostic stdout. This is the
official [stdio transport profile](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports).
No HTTP/network listener or detached task is created.

Default message cap is 1 MiB, catalog cap 64 tools, ID cap 256 bytes. Limits are
caller-selected; ID caps must fit conservative JSON escaping overhead. Domain
result objects are checked against one eighth of the message cap before creating
compatible text, and full response sizes are checked again. This intentionally
reserves JSON escaping/envelope space. A result-limit failure says to inspect
operation state: an operation may already have completed, and the protocol never
claims rollback or invents a success receipt. Backend source/job/state budgets,
idempotency and commit semantics remain in the native owner.

This is synchronous request handling without task/cancellation execution,
pagination, resources, prompts, sampling, HTTP, reconnect/replay or authentication
transport. Those capabilities are not advertised. There is no claim of complete
MCP feature coverage or cooperative production readiness.

## Executed checks

```powershell
cargo test --manifest-path docs/qa/implementation-2026-10-25/batch-3/mcp/harness/Cargo.toml --offline --locked
```

Final execution compiles the exact candidate `src/lib.rs` with authoring enabled
and engine-tools disabled in a small standalone workspace: 16 tests passed,
0 failed, 0 doc tests. Only cached serde/serde_json and their macro dependencies
were built. This exercises new lib/module feature routing without full core or
engine builds; actual workspace dependency resolution and root native gateway
integration remain parent's checks. The harness package has a different package
version, so tests do not claim the production serverInfo version was executed.

The initial run had 11 passing tests and one bad framing-test limit fixture:
512-byte message budget conflicted with the default 256-byte ID budget. The fixture
now sets a valid small ID budget. Subsequent tests added duplicate-key, metadata,
catalog and escaped-ID coverage; the final candidate/lib routing run passed 16/16.
An intermediate compile emitted a cache hard-link warning and copied files;
the final routing run exited 0. `verification.json` records the final result.

No authenticated user context, native authoring effect, HMC save/reopen, production
principal, human review or payment was exercised by these fixtures. No Computer
Use, sibling edit, service publication or money movement occurred. IP-03 C1–C8,
all 120 production packets and complete model/master/consumer outcomes stay open.
