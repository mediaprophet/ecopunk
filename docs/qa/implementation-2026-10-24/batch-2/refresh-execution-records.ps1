param([int]$LiveBuilds = 0)
$ErrorActionPreference = 'Stop'
if ($LiveBuilds -lt 0) { throw 'Invalid live command count' }
$planning = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../../../planning'))
$tech = Join-Path $planning 'tech'
function Read-Record($path) { Get-Content -LiteralPath $path -Raw | ConvertFrom-Json -AsHashtable }
function Write-Record($path, $record) {
    [IO.File]::WriteAllText($path, ($record | ConvertTo-Json -Depth 70) + [Environment]::NewLine, [Text.UTF8Encoding]::new($false))
}
function Pin($path) { (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant() }
function Refresh-Requirements($record) {
    foreach ($pin in $record.supporting_requirement_documents) {
        $pin.sha256 = Pin (Join-Path $tech $pin.path)
    }
    $record.implementation_plan.version = '0.5'
    $record.implementation_plan.sha256 = Pin (Join-Path $tech $record.implementation_plan.path)
}
$queuePath = Join-Path $planning 'implementation-queue.json'
$queue = Read-Record $queuePath
$queue.version = '2026-10-24.3'
foreach ($blocker in $queue.observed_blockers) {
    if ($blocker.id -like 'RUN05*') {
        $blocker.status = 'reviewed bounded source repair applied; target verification recorded separately; broader acceptance open'
        $blocker.evidence = '../qa/implementation-2026-10-24/batch-2/README.md'
    }
}
foreach ($job in $queue.jobs) {
    if ($job.id -in @('ECO-018','ECO-034','ECO-044','ECO-045','ECO-050')) {
        $job.dispatch_status = 'bounded canonical/session/journal/native-mount implementation under RUN-05/06/07; full packet acceptance open; see batch-2 evidence'
    }
}
$batchRef = '../qa/implementation-2026-10-24/batch-2/README.md'
if ($batchRef -notin $queue.bounded_execution_refs) { $queue.bounded_execution_refs += $batchRef }
Write-Record $queuePath $queue

$templatePath = Join-Path $tech 'ecopunk-production-claims.template.json'
$template = Read-Record $templatePath
if (@($template.packets | Where-Object { $_.status -ne 'open' -or $_.milestone_results.Count -ne 0 }).Count -ne 0) {
    throw 'Starter unexpectedly contains execution claims'
}
$template.version = '2026-10-24.9'
Refresh-Requirements $template
Write-Record $templatePath $template

$actualPath = Join-Path $tech 'ecopunk-production-claims.json'
$actual = Read-Record $actualPath
$actual.version = '2026-10-24.3'
Refresh-Requirements $actual
$actual.source_template.version = $template.version
$actual.source_template.sha256 = Pin $templatePath
if (@($actual.packets | Where-Object status -ne 'open').Count -ne 0 -or $actual.packets.Count -ne 120) {
    throw 'Broad packet status/count changed'
}
$evidence = '../../qa/implementation-2026-10-24/batch-2/applied-sources.json'
$review = '../../qa/implementation-2026-10-24/batch-2/runtime-review.md'
$milestones = @(
    @{ids=@('ECO-018','ECO-034'); id='RUN-05A'; scope='Canonical full-input cell parser; five applied regression passes'; status='bounded_source_repair_verified'; remaining='Full parser/language-profile coverage and all broader packet acceptance'; refs=@($evidence,'../../qa/implementation-2026-10-24/batch-2/parser/applied.log')},
    @{ids=@('ECO-018','ECO-034','ECO-044'); id='RUN-05B'; scope='Canonical transitive effects and persistent checked source session; native Poet integration'; status='bounded_repair_applied_with_separate_target_receipt'; remaining='Durable shared projects, checked replacement/live jobs, graphical cooperation and complete language/profile acceptance'; refs=@($evidence,'../../qa/implementation-2026-10-24/batch-2/runtime/actual-verification.json')},
    @{ids=@('ECO-034','ECO-044'); id='RUN-06B'; scope='Existing native Studio source undo/redo and atomic expected-revision admission; ten focused passes'; status='bounded_source_journal_repair_verified'; remaining='Current final-integration qualification, durable checkpoint/reopen, authorized cooperative workflow and complete source-to-HMC production bindings'; refs=@($evidence,'../../qa/implementation-2026-10-24/batch-2/native-3d/verification.json','../../qa/implementation-2026-10-24/batch-2/consumer-checks.json')},
    @{ids=@('ECO-034','ECO-044'); id='RUN-07A-STORE'; scope='Applied opaque incarnation/revision stamps, fallible edits, atomic source admission and cheap current-stamp validation'; status='bounded_stamped_source_primitives_applied'; remaining='Coherent final-core integration, project authority, durable recovery and all broader cooperation acceptance'; refs=@('../../qa/implementation-2026-10-24/batch-2/native-poet/store-current-verification.json','../../qa/implementation-2026-10-24/batch-2/native-3d-review.md')},
    @{ids=@('ECO-034','ECO-044','ECO-045'); id='RUN-07A-NATIVE'; scope='Applied actual Poet optional native Aether editor with real geometry Host, stamped preview completion, bounded explicit read-only execution rebuild and targeted browser-binding alignment'; status='bounded_native_integration_applied_pending_runtime'; remaining='Coherent tests and actual native window/frame proof; material authoring, verified HMC retention, durable contributor/reviewer workflow, high-grade production and complete IP-03 acceptance'; refs=@('../../qa/implementation-2026-10-24/batch-2/native-poet/applied-verification.json','../../qa/implementation-2026-10-24/batch-2/integrated-sources.json','../../qa/implementation-2026-10-24/batch-2/native-poet-review.md')}
)
foreach ($milestone in $milestones) {
    foreach ($packetId in $milestone.ids) {
        $packet = $actual.packets | Where-Object id -eq $packetId
        $item = [ordered]@{
            id=$milestone.id; scope=$milestone.scope; status=$milestone.status
            evidence_refs=$milestone.refs; remaining=$milestone.remaining; human_acceptance_claimed=$false
            evidence_pins=@($milestone.refs | ForEach-Object { @{path=$_; sha256=(Pin (Join-Path $tech $_))} })
        }
        if ($milestone.id -eq 'RUN-05B') { $item.review_refs=@($review); $item.review_pins=@(@{path=$review; sha256=(Pin (Join-Path $tech $review))}) }
        $packet.milestone_results = @($packet.milestone_results | Where-Object id -ne $milestone.id) + @($item)
    }
}
$batchReadme='../../qa/implementation-2026-10-24/batch-2/README.md'
foreach ($path in @($batchReadme,$evidence)) {
    $oldPin=$actual.supporting_requirement_documents | Where-Object path -eq $path
    if ($null -eq $oldPin) { $actual.supporting_requirement_documents += @{path=$path; sha256=(Pin (Join-Path $tech $path))} }
}
$actual.execution_summary.batch = '2026-10-24 bounded implementation batches 1 and 2'
$actual.execution_summary.remaining_priority = 'Real native Poet/Aether geometry/material authoring, source/revision recovery and cooperative source-to-HMC workflow; exceptional editable masters and reviewed game-demo derivatives'
$actual.execution_summary.batch_2_evidence = $batchReadme
$actual.execution_summary.live_builds = $LiveBuilds
Write-Record $actualPath $actual
Write-Output 'Refreshed queue, blank starter and actual ledger; all 120 broad packets remain open.'
