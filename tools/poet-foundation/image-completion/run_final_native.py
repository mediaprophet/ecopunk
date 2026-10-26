"""Run only the final two native image checks using Cargo's just-built harnesses.

Coordinator invokes this after the serial --no-run build, with Cargo idle.
No native window or production asset is launched.
"""
from pathlib import Path
import hashlib
import json
import os
import re
import subprocess
from verify_saved import main as verify_sources

ROOT = Path('C:/github/ecopunk')
QA = ROOT / 'docs/qa/implementation-2026-10-26/image-completion'
build = json.loads((QA / 'native-final-build-attempt-2-result.json').read_text(encoding='utf-8-sig'))
assert build['exit_code'] == 0
assert not (ROOT / 'target/coordination/native-check.lock').exists(), 'Cargo still active'
verify_sources()
text = (QA / 'native-final-build-attempt-2.log').read_text(encoding='utf-8-sig')
artifacts = {}
for item in re.findall(r'Executable [^\r\n]*\(([^\r\n)]+\.exe)\)', text):
    path = Path(item.replace('\\', '/'))
    if not path.is_absolute():
        path = ROOT / path
    path = path.resolve()
    assert path.is_relative_to(ROOT / 'target/debug/deps')
    for prefix in ('poet_cli-', 'foundation_image_completion-'):
        if path.name.startswith(prefix):
            artifacts[prefix] = path
assert set(artifacts) == {'poet_cli-', 'foundation_image_completion-'}, artifacts
binary = ROOT / 'target/debug/poet.exe'
assert binary.is_file()
for manifest in ('manifest-core.json', 'manifest-poet.json'):
    data = json.loads((ROOT / 'tools/poet-foundation/image-completion/work/integration' / manifest).read_text())
    assert binary.stat().st_mtime >= max(Path(row['destination']).stat().st_mtime for row in data['files']), 'Native executable predates saved source'
env = dict(os.environ, QUALIA_RENDER_WGPU_BACKEND='dx12')
for stem, exe, args in [
    ('core-final', artifacts['foundation_image_completion-'], ['--nocapture', '--test-threads=1']),
    ('gpu-refusal-final', artifacts['poet_cli-'], ['--ignored', '--exact', 'graphical::native_project::tests::texture_effect_workflow_tests::gpu_tests::image_completed_tests::image_completed_workflow_refusals_preserve_source_history', '--nocapture', '--test-threads=1']),
]:
    command = [str(exe), *args]
    with (QA / (stem + '.log')).open('w', encoding='utf-8', newline='\n') as log:
        result = subprocess.run(command, cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT)
    receipt = {'command': command, 'exit_code': result.returncode, 'harness_sha256': hashlib.sha256(exe.read_bytes()).hexdigest(), 'compiled_by': 'native-final-build-attempt-2.log', 'execution': 'root serial; Cargo idle', 'backend': 'dx12'}
    (QA / (stem + '-result.json')).write_text(json.dumps(receipt, indent=2) + '\n', encoding='utf-8', newline='\n')
    print(stem, 'exit', result.returncode, flush=True)
    if result.returncode:
        raise SystemExit(result.returncode)
verify_sources()
