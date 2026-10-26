from pathlib import Path
root=Path(__file__).parent;prior=root.parent/'batch-4'
(root/'run-final-pixels.ps1').write_text((prior/'run-final-pixels.ps1').read_text(encoding='utf-8-sig'),encoding='utf-8')
folder=root/'direct-cli';folder.mkdir(exist_ok=True)
script=(prior/'direct-cli/link_and_check.py').read_text(encoding='utf-8-sig').replace('poet-batch4','poet-batch5')
# Verify the real native-project entry in the same actual executable. The empty
# host-owned fixture contains no person/profile, authority journal or HMC source.
marker='    print("Direct actual-source CLI linked; help, no-launch source check, strict MCP/unbound denial passed.")'
assert marker in script
extra='''    public_root = HERE / "public-project-only"
    public_root.mkdir()
    project_stdout, _, project_receipt = child([str(OUTPUT), "studio", "--native-project", str(public_root), "--check-project"], "native-project-check", timeout=30)
    expected = "no vault opened, key adopted, source read, enrollment or window attempted"
    if expected not in project_stdout or list(public_root.iterdir()):
        raise RuntimeError("Native public-config-only entry did not preserve its no-activation branch")
    public_check = {"exit_code": 0, "public_config_only": True, "source_files_opened": False,
                    "os_credential_accessed": False, "adoption_created": False, "gui_launched": False,
                    "scope": "Executed public-configuration branch, inspected source path and empty-directory witness; not an OS provider/system-call audit",
                    "artifact_sha256": sha(OUTPUT), "input_snapshot": snapshot, "source_unchanged": True,
                    "actual_invocation_receipt": str(HERE / "native-project-check.json"), "checked_utc": now()}
    (BUILD / "direct-cli/native-project-check.json").write_text(json.dumps(public_check, indent=2) + "\\n", encoding="utf-8")
'''
script=script.replace(marker,extra+marker)
(folder/'link_and_check.py').write_text(script,encoding='utf-8')
print('Prepared final actual GPU/CLI/native public-entry checks; none executed.')
