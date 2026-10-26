$ErrorActionPreference = 'Stop'
$authority = 'C:/github/QualiaDB/crates/qualia-core-db/src/poet_host/native_authority'
$invoke = 'C:/github/QualiaDB/crates/qualia-core-db/src/poet_host/invoke'
$native = 'C:/github/Poet/native/src'
$paths = @(
    "$authority/mod.rs", "$authority/admission.rs", "$authority/session.rs",
    "$authority/journal.rs", "$authority/records.rs", "$authority/workflow.rs",
    "$authority/workflow_tests.rs", "$authority/tests.rs",
    "$invoke/studio_store.rs", "$invoke/studio_stamped.rs", "$invoke/studio_journal.rs",
    "$invoke/studio_checkpoint/native_io.rs", "$invoke/studio_checkpoint/tests.rs",
    'C:/github/QualiaDB/crates/qualia-core-db/src/identity/key_vault.rs',
    'C:/github/QualiaDB/crates/qualia-core-db/src/identity/key_vault/native_secure_lock.rs',
    'C:/github/QualiaDB/crates/qualia-core-db/Cargo.toml',
    'C:/github/QualiaDB/Cargo.lock', 'C:/github/Poet/Cargo.lock',
    'C:/github/Poet/native/Cargo.toml',
    "$native/main.rs", "$native/graphical/mod.rs", "$native/graphical/entry.rs",
    "$native/graphical/native_project.rs", "$native/graphical/native_project_workflow_tests.rs",
    "$native/graphical/session_host.rs", "$native/graphical/draft_buffer.rs",
    "$native/graphical/editor_contract.rs", "$native/graphical/jobs.rs",
    "$native/graphical/jobs_tests.rs", "$native/graphical/ui.rs",
    "$native/graphical/material_controls.rs", "$native/graphical/cooperative_controls.rs",
    "$native/graphical/cooperation/mod.rs", "$native/graphical/cooperation/native_owner.rs",
    'C:/github/QualiaDB/crates/qualia-cooperative-core/src/authoring/policy.rs',
    'C:/github/QualiaDB/crates/qualia-cooperative-core/src/authoring/records.rs'
)
$paths += @(Get-ChildItem -LiteralPath "$authority/enrollment" -Filter '*.rs' -File | ForEach-Object FullName)
$paths += @(Get-ChildItem -LiteralPath "$native/graphical/native_entry" -Filter '*.rs' -File | ForEach-Object FullName)
$paths += @(Get-ChildItem -LiteralPath "$native/graphical/cooperative_workflow" -Filter '*.rs' -File | ForEach-Object FullName)
$files = @($paths | Sort-Object -Unique | ForEach-Object {
    $item = Get-Item -LiteralPath $_
    [ordered]@{ path = $item.FullName; bytes = $item.Length; sha256 = (Get-FileHash -LiteralPath $item.FullName -Algorithm SHA256).Hash.ToLowerInvariant() }
})
$snapshotRef = Get-Content -LiteralPath (Join-Path $PSScriptRoot '../latest-source-snapshot.json') -Raw | ConvertFrom-Json
$snapshot = Get-Content -LiteralPath $snapshotRef.path -Raw | ConvertFrom-Json
$index = @{}
foreach ($source in $snapshot.sources) { $index[$source.path.Replace('/', '\').ToLowerInvariant()] = $source.sha256 }
$mismatches = @()
$outsideSnapshot = @()
foreach ($file in $files) {
    $key = $file.path.Replace('/', '\').ToLowerInvariant()
    if (!$index.ContainsKey($key)) { $outsideSnapshot += $file.path }
    elseif ($index[$key] -ne $file.sha256) { $mismatches += $file.path }
}
if ($mismatches.Count -gt 0) { throw "Review inputs differ from supervisor freeze: $($mismatches -join ', ')" }
$record = [ordered]@{
    kind = 'independent-source-review-inputs'
    captured_utc = [DateTime]::UtcNow.ToString('o')
    scope = 'RUN-10 enrollment, source workflow, configured native operator entry, dynamic session host, UI and job publication; source review only'
    qualifications = @('No reviewer build/test/GPU/GUI or OS credential operation', 'Dependency manifests hashed for provenance, not a full dependency audit', 'Unchanged batch-4 package/resource review reused within its declared limits')
    supervisor_snapshot = $snapshotRef
    snapshot_comparison = [ordered]@{ matched = $files.Count - $outsideSnapshot.Count; mismatches = $mismatches; supporting_inputs_outside_snapshot = $outsideSnapshot }
    files = $files
}
$record | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $PSScriptRoot 'reviewed-inputs.json') -Encoding utf8NoBOM
Write-Output "Captured $($files.Count) reviewed source/dependency inputs; $($files.Count - $outsideSnapshot.Count) match supervisor freeze, $($mismatches.Count) mismatches"
