from pathlib import Path
import json, hashlib

root = Path(__file__).parent
target = Path('C:/github/qualiaDB/crates/vibe/src/session/mod.rs')
original = target.read_bytes()
source = original.decode('utf-8')
source = source.replace('mod values;', 'mod values;\n#[cfg(test)]\nmod validation_tests;')
needle = '''        let candidate = self.prepare(source).map_err(|e| self.error(e, false))?;
        self.execute(candidate, host, budget)
    }
    pub fn execute<H: Host>('''
replacement = '''        self.submit_validated(source, host, budget, |_| Ok(()))
    }
    /// Validate evaluated output before promoting any tentative local values,
    /// definitions or source. Host effects already attempted are not rolled back.
    pub fn submit_validated<H: Host, F: FnOnce(&Value) -> Result<(), Diagnostic>>(
        &mut self,
        source: &str,
        host: &mut H,
        budget: Budget,
        validate: F,
    ) -> Result<SubmissionResult, SubmissionError> {
        let candidate = self.prepare(source).map_err(|e| self.error(e, false))?;
        self.execute_validated(candidate, host, budget, validate)
    }
    pub fn execute<H: Host>('''
assert source.count(needle) == 1
source = source.replace(needle, replacement)
needle = '''    ) -> Result<SubmissionResult, SubmissionError> {
        if candidate.owner != self.owner'''
replacement = '''    ) -> Result<SubmissionResult, SubmissionError> {
        self.execute_validated(candidate, host, budget, |_| Ok(()))
    }
    /// The validator receives actual evaluated output once, after interpreter
    /// checks and before promotion. It may recheck native authority while its
    /// owner transaction is held; no JSON receipt is treated as authority.
    pub fn execute_validated<H: Host, F: FnOnce(&Value) -> Result<(), Diagnostic>>(
        &mut self,
        candidate: CheckedSubmission,
        host: &mut H,
        budget: Budget,
        validate: F,
    ) -> Result<SubmissionResult, SubmissionError> {
        if candidate.owner != self.owner'''
assert source.count(needle) == 1
source = source.replace(needle, replacement)
needle = '        self.env = tentative;'
assert source.count(needle) == 1
source = source.replace(needle, '        validate(&value).map_err(|e| self.error(e, true))?;\n' + needle)
(root/'candidate/mod.rs').write_text(source, encoding='utf-8', newline='\n')
mutations = []
for name, data, old in [('mod.rs',source.encode(),original),('validation_tests.rs',(root/'candidate/validation_tests.rs').read_bytes(),None)]:
    mutations.append(dict(target=str(target.parent/name), candidate='candidate/'+name,
        original_sha256=hashlib.sha256(old).hexdigest() if old else None,
        candidate_sha256=hashlib.sha256(data).hexdigest()))
(root/'apply-manifest.json').write_text(json.dumps(dict(scope='Pre-promotion canonical Vibe output validation',mutations=mutations),indent=2))
