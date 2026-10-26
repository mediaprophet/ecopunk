from pathlib import Path
root = Path('C:/github/QualiaDB/crates/qualia-core-db/src/poet_host/invoke')
out = Path(__file__).parent/'candidate/store'
for name in ('studio_journal.rs', 'studio_stamped.rs', 'studio_store.rs'):
    original = (root/name).read_text(encoding='utf-8')
    candidate = (out/name).read_text(encoding='utf-8')
    if name == 'studio_journal.rs':
        addition = candidate[candidate.index('/// Actual retained undo source'):candidate.index('impl SourceState {')]
        original = original.replace('impl SourceState {', addition+'impl SourceState {', 1)
    elif name == 'studio_stamped.rs':
        addition = candidate[candidate.index('/// Inspect the existing undo predecessor'):candidate.index('/// Atomic admission')]
        original = original.replace('/// Atomic admission', addition+'/// Atomic admission', 1)
        original += '\n'+candidate[candidate.index('#[cfg(test)]\nmod predecessor_tests'):]
    else:
        original = original.replace('pub use journal::SourceJournalLimits;', 'pub use journal::{EditableSourceSnapshot, SourceJournalLimits};')
        original = original.replace('redo_artifact_stamped, undo_artifact_stamped, StoreStamp};', 'redo_artifact_stamped, undo_artifact_stamped, previous_source_snapshot, StoreStamp};')
    (out/name).write_text(original, encoding='utf-8')
