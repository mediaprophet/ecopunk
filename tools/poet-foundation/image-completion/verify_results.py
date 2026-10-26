"""Check retained image qualification evidence; does not run Cargo or grant acceptance."""
from pathlib import Path
import hashlib
import json
import re
import struct
import sys
import tomllib
from verify_saved import main as verify_sources

ROOT = Path('C:/github/ecopunk')
QA = ROOT / 'docs/qa/implementation-2026-10-26/image-completion'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    verify_sources()
    runs = {}
    # A caller explicitly selects the successful GPU attempt. Failed attempts stay intact.
    gpu = sys.argv[1]
    for label, stem in [('core', 'core-acceptance-final'), ('core_owner', 'core-owner-attempt-5'),
                        ('poet', 'poet-library-attempt-2'),
                        ('gpu_refusal', 'gpu-refusal-final'), ('gpu_offsets', 'gpu-offset-only-final'),
                        ('legacy_gpu', 'legacy-gpu')]:
        result = json.loads((QA / (stem + '-result.json')).read_text(encoding='utf-8-sig'))
        assert result['exit_code'] == 0, (label, result)
        log = QA / (stem + '.log')
        text = log.read_text(encoding='utf-8-sig', errors='strict')
        summaries = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', text)
        assert summaries, f'No executed test summary in {log}'
        passed, failed, ignored = map(int, summaries[-1])
        assert passed > 0 and failed == 0, (label, summaries[-1])
        if label == 'core':
            assert passed >= 20 and ignored == 0
        elif label == 'poet':
            assert passed >= 289
            assert len(re.findall(r'^test .*image_completed_.* \.\.\. ok$', text, re.M)) >= 18
        elif label == 'gpu_refusal':
            assert passed == 1 and ignored == 0
            assert 'image_completed_workflow_refusals_preserve_source_history ... ok' in text
        elif label == 'gpu_offsets':
            assert passed == 1 and ignored == 0
            assert 'image_completed_workflow_offsets_without_masks_change_native_gpu ... ok' in text
        runs[label] = dict(result, passed=passed, failed=failed, ignored=ignored,
                           log=str(log), log_sha256=sha(log))

    # Preserve the original failed batch honestly. Its positive workflow passed;
    # the only failed test is rerun separately after repairing its hold/rebuild
    # assumption. Avoid repeating the unchanged expensive positive workflow.
    first = json.loads((QA / (gpu + '-result.json')).read_text(encoding='utf-8-sig'))
    first_log = QA / (gpu + '.log')
    text = first_log.read_text(encoding='utf-8-sig')
    assert first['exit_code'] == 101
    assert 'image_completed_workflow_parent_child_passed' in text
    assert 'image_completed_workflow_filters_blends_masks_hmc_and_fresh_process ... ok' in text
    assert 'test result: FAILED. 1 passed; 1 failed; 0 ignored;' in text
    stage = ROOT / 'tools/poet-foundation/image-completion/work/integration'
    repair = json.loads((stage / 'manifest-poet-refusal.json').read_text())
    marker = b'#[ignore = "explicit signed native GPU image completion refusal qualification"]'
    current = Path(repair['files'][0]['destination']).read_bytes()
    assert hashlib.sha256(current.split(marker)[0]).hexdigest() == repair['positive_fixture_prefix_sha256']
    runs['gpu_positive'] = {'command': first['command'], 'batch_exit_code': 101,
                            'selected_test_result': 'passed', 'log': str(first_log),
                            'log_sha256': sha(first_log), 'failed_sibling_repaired_and_rerun': True,
                            'positive_fixture_prefix_sha256': repair['positive_fixture_prefix_sha256']}

    folder = QA / gpu
    witness = json.loads((folder / 'image-completed-witness.json').read_text(encoding='utf-8'))
    assert len(witness['cases']) == 13
    files = []
    for name, expected in witness['evidence_files_sha256'].items():
        assert Path(name).name == name
        path = folder / name
        assert sha(path) == expected, name
        row = {'path': str(path), 'sha256': expected, 'bytes': path.stat().st_size}
        if name.endswith('.png'):
            data = path.read_bytes()
            assert data[:8] == b'\x89PNG\r\n\x1a\n'
            row['dimensions'] = struct.unpack('>II', data[16:24])
        files.append(row)
    assert sum(r['path'].endswith('.hmc') for r in files) == 3
    assert sum(r['path'].endswith('.png') for r in files) >= 17
    offset_files = []
    for part in ('baseline', 'positive', 'negative', 'original-only'):
        path = QA / 'gpu-offset-only-final' / f'image-offset-{part}-gpu.png'
        assert path.read_bytes()[:8] == b'\x89PNG\r\n\x1a\n'
        offset_files.append({'path': str(path), 'sha256': sha(path)})
    child = json.loads((folder / 'image-child-recovered.json').read_text(encoding='utf-8'))
    assert child, 'Missing real child recovery witness'

    previous = json.loads((ROOT / 'docs/qa/implementation-2026-10-26/poet-texture-effects/gpu-qualification/verification.json').read_text())
    lock = Path('C:/github/poet/Cargo.lock')
    assert sha(lock) == previous['dependency_resolution']['lock_sha256'], 'Dependency lock changed'
    entries = tomllib.loads(lock.read_text())['package']
    selected = [{k: p.get(k) for k in ('name', 'version', 'source')}
                for p in entries if p['name'] in ('etalon', 'wgpu', 'naga')]
    binary = ROOT / 'target/debug/poet.exe'
    report = {'scope': 'Fifteen selected image functions and their native authoring integration',
              'runs': runs, 'gpu_artifacts': files, 'isolated_offset_gpu_artifacts': offset_files,
              'dependencies': {'lock_sha256': sha(lock), 'selected': selected,
                               'caveat': 'Resolved Etalon Git source; not the local Etalon worktree'},
              'native_binary': {'path': str(binary), 'sha256': sha(binary)},
              'asset_production': 'HOLD', 'native_window_human_acceptance': 'UNASSESSED',
              'cuda_hardware_qualification': 'UNAVAILABLE: driver absent; optional-backend tests may return early and do not establish CUDA execution',
              'peer_delivery_reliability': 'One earlier full-suite timeout, one isolated pass; cause remains unconfirmed in mesh-review.md',
              'limits': 'Encoded RGBA8 artistic operations; synthetic 8x8 source / native GPU verification; no production or biomedical quality claim'}
    (QA / 'verification.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8', newline='\n')
    print('Verified all retained image qualification evidence; production remains HOLD')


if __name__ == '__main__':
    main()
