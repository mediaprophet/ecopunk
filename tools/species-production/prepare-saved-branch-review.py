"""Create an integrity-sealed review derivative of the preserved branch study.

Only bytes52..56 (whole-file CRC32C) may change. Geometry/material data are preserved.
Reference: QualiaDB container_10d/integrity.rs and crc32c.rs. Not an asset acceptance gate.
"""
from pathlib import Path
import hashlib
import json

root = Path(__file__).resolve().parents[2]
source = root / "assets/production-studies/eucalypt-spray-r1/revisions/2026-10-29/eucalypt-spray.10d"
output = root / "viewer/model-artifacts/saved-branch-review.10d"
table = []
for number in range(256):
    c = number
    for _ in range(8):
        c = (c >> 1) ^ (0x82F63B78 if c & 1 else 0)
    table.append(c)

def crc32c(data):
    crc = 0xFFFFFFFF
    for byte in data:
        crc = table[(crc ^ byte) & 255] ^ (crc >> 8)
    return crc ^ 0xFFFFFFFF

assert crc32c(b"123456789") == 0xE3069283
original = source.read_bytes()
assert len(original) >= 64
sealed = bytearray(original)
old_crc = int.from_bytes(sealed[52:56], "little")
sealed[52:56] = bytes(4)
new_crc = crc32c(sealed)
sealed[52:56] = new_crc.to_bytes(4, "little")
assert original[:52] == sealed[:52] and original[56:] == sealed[56:]
output.parent.mkdir(parents=True, exist_ok=True)
output.write_bytes(sealed)
record = dict(source=str(source), output=str(output), source_sha256=hashlib.sha256(original).hexdigest(),
              output_sha256=hashlib.sha256(sealed).hexdigest(), old_crc32c=old_crc,
              computed_crc32c=new_crc, changed_byte_range=[52,56],
              result="Integrity field only; original preserved; browser admission and visual review separate")
(root / "docs/qa/implementation-2026-10-29/species-production-round1/saved-branch-integrity.json").write_text(json.dumps(record, indent=2)+"\n")
print(json.dumps(record, indent=2))
