# Synthetic texture/effect evidence

These are actual canonical encoded HMC resources retained by the passing signed native/MCP test. They are **8×8 engineering calibration images**, not production assets or biological references. The native project test retains an editable paint layer over the imported source, then applies Invert; a Sepia step remains present but disabled. The reopened output must have the same encoded digest as the filtered output.

| Imported immutable source | Paint layer + active Invert | Fresh-process reopened result |
|---|---|---|
| [original.png](original.png) | [effects.png](effects.png) | [reopened.png](reopened.png) |

[Witness](witness.json) records actual project/document/source identifiers and original/output/package digests. It does not establish human review, permission from a digest, GPU appearance or biological quality. Tests separately compare exact decoded pixels, retained recipe/history and original-resource metadata. Asset production remains HOLD.
