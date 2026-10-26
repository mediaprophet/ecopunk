"""Stage only exact selected legacy test extraction and new public routing."""
import hashlib
import json
from pathlib import Path
import re

root = Path(__file__).resolve().parent
source = Path("C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/mod.rs")
original = (root / "originals/mod.rs").read_bytes()
text = original.decode("utf-8")
names = ["dispatch_tick_hook_fires", "g_tick_hook_with_pulse_publish_through_snapshot",
         "t42_hid_vibescript_poll", "t42_hid_vibescript_poll_empty",
         "test_crypto_ed25519_generate_sign_verify_roundtrip", "crypto_sign_fail_closed"]
extracted = []
for name in names:
    pattern = r"(?ms)^    #\[test\]\r?\n    fn " + re.escape(name) + r"\(\) \{.*?^    \}\r?\n"
    matches = list(re.finditer(pattern, text))
    if len(matches) != 1:
        raise RuntimeError("Expected exactly one selected original test: " + name)
    extracted.append({"name":name, "original":matches[0].group(0),
                      "new_module":"poet_host::scoped_effects::tests"})
    text = text[:matches[0].start()] + text[matches[0].end():]
newline = "\r\n" if b"\r\n" in original else "\n"
if text.count("mod values;") != 1:
    raise RuntimeError("Owning module route is ambiguous")
text = text.replace("mod values;", "mod values;" + newline + "pub mod scoped_effects;", 1)
(root / "candidate/mod.rs").write_bytes(text.encode("utf-8"))
(root / "extracted-test-history.json").write_text(json.dumps(extracted, indent=2) + "\n", encoding="utf-8")

def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()
mutations = [{"target":str(source),"original_sha256":sha(root/"originals/mod.rs"),
              "candidate":"candidate/mod.rs","candidate_sha256":sha(root/"candidate/mod.rs")}]
for path in sorted((root/"candidate/scoped_effects").glob("*.rs")):
    mutations.append({"target":str(source.parent/"scoped_effects"/path.name),"original_sha256":None,
                      "candidate":path.relative_to(root).as_posix(),"candidate_sha256":sha(path)})
(root/"apply-manifest.json").write_text(json.dumps({"scope":"Native host effect grants and canonical 0.1 conformance fixtures",
    "applied":False,"mutations":mutations},indent=2) + "\n",encoding="utf-8")
print(json.dumps({"staged_targets":len(mutations),"selected_original_tests_extracted":names}))
