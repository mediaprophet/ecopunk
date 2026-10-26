# Literal native UI text repair

Source-only check of the six Round 4 installed native Poet UI files selected
from the Round 4 install manifest: package_controls.rs,
package_georaster_create_view.rs, package_georaster_edit_view.rs,
package_review_controls.rs, native_entry/package_grant_controls.rs and
native_entry/ui.rs. No unrelated source tree or asset was searched.

Two files contain literal UTF-8 mojibake. package_review_controls.rs has nine
double-misencoded middle-dot separators. package_controls.rs has two misencoded
middle-dot separators and one misencoded apostrophe. The staged candidate replaces
only those exact byte sequences with ASCII `|` or `'`, respectively. Original
labels and meaning, line endings, all other bytes and valid existing Unicode
remain unchanged. The four other checked UI files need no repair.

The installer-compatible manifest uses target, stage, before_sha256 and
after_sha256 keys. Full preimages/postimages are under before/ and after/;
inspection.json pins the six inspected live sources and records Unicode escapes;
replacement-details.json records the twelve literal replacements. The manifest
SHA256 is `b03f0e9c70a3495d013eb0fcac2e667bc0e19ba068d4528bdc3c30bd35394c89`.

Only this new staging folder was written. No live Poet source, package, asset,
GUI, credential, build or test was changed or executed. Root owns guarded
installation after the currently active check and subsequent source-pin/build
qualification. This text repair implies no graphical/human acceptance or passed
foundation/model completion gate.
