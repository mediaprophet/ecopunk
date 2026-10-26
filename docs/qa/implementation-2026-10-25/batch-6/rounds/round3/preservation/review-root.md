# Independent structural retention review

Root reviewed the staged policy, opaque inspector/copy/replacement implementation,
section delegation and test cases. Strict section interpretation is unchanged.
Bounds, metadata/payload overlap, duplicate tags, padding, stride and all CRCs are
checked before copying; all capacity/policy errors precede the write commit.
The changed-byte path additionally refuses unknown flags, descriptor metadata,
unimplemented sections and uninterpreted nonzero tail. Its same-length layout
rule avoids relocation ambiguity. No blocking source finding in this scope.

The API validates structural retention, not the replacement's typed correctness
or known semantic dependencies. It cannot authorize unknown extensions or safely
edit unresolved dependencies. Both parsing work and output are bounded by the
explicit caller limits and the existing hard section cap. Allocation claims need
the recorded native counter tests. The larger section module remains oversized;
this addition is isolated child delegation, not a decomposition-completion claim.
