"""Human-authorized internal Poet library absorption; preserve historical sources."""
from pathlib import Path
import datetime, hashlib, json

poet = Path('C:/github/poet')
staged = Path('C:/github/craft')
qa = Path(__file__).parent
geometry = poet/'crates/geometry'
authoring = poet/'crates/spatial-authoring'
for target in [geometry, authoring]:
    if target.exists():
        raise SystemExit(f'Existing target must be inspected/preserved: {target}')
rows = []
def copy_record(source, destination, transform=lambda text:text):
    before=source.read_bytes()
    after=transform(before.decode('utf-8-sig')).encode('utf-8')
    destination.parent.mkdir(parents=True,exist_ok=True)
    if destination.exists():
        raise RuntimeError(f'Refusing to overwrite {destination}')
    destination.write_bytes(after)
    rows.append(dict(source=str(source),target=str(destination),source_sha256=hashlib.sha256(before).hexdigest(),target_sha256=hashlib.sha256(after).hexdigest()))

for source in sorted((staged/'crates/geom/src').glob('*.rs')):
    # Preserve the copied legacy sources byte-for-byte, including historical comments.
    destination=geometry/'src'/source.name
    destination.parent.mkdir(parents=True,exist_ok=True)
    data=source.read_bytes();destination.write_bytes(data)
    digest=hashlib.sha256(data).hexdigest()
    rows.append(dict(source=str(source),target=str(destination),source_sha256=digest,target_sha256=digest))
for source in sorted((staged/'crates/authoring/src').glob('*.rs')):
    copy_record(source,authoring/'src'/source.name,
        lambda text:text.replace('cadcraft_geom','poet_geometry').replace('CraftError','AuthoringError').replace('CraftPart','AssemblyPart'))

license_source=Path('C:/github/poet-cadcraft/LICENSE-MIT')
for target in [geometry,authoring]:
    copy_record(license_source,target/'LICENSE-MIT')

common='''version = "0.4.0"
edition = "2024"
license = "MIT"
rust-version = "1.90"

[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
dbg_macro = "warn"
todo = "warn"
too_many_arguments = "allow"
type_complexity = "allow"
'''
(geometry/'Cargo.toml').write_text('[package]\nname = "poet-geometry"\ndescription = "Poet reusable f64 geometry and profile helpers"\n'+common+'''
[dependencies]
serde = { workspace = true }
''')
(authoring/'Cargo.toml').write_text('[package]\nname = "poet-spatial-authoring"\ndescription = "Poet source-backed metric profiles and identified spatial assembly helpers"\n'+common+'''
[dependencies]
poet-geometry = { path = "../geometry" }
qualia-core-db = { path = "C:/github/qualiaDB/crates/qualia-core-db", default-features = false, features = ["profile_target_1024", "zk-culling", "gpu-runtime", "gpu-native-portable", "gpu-native-dx12", "wgsl-forge", "privacy-he", "sanctuary-crypto"] }
''')

native=poet/'native/src/graphical'
for name in ['geometry_host.rs','craft_host.rs']:
    source=native/name
    backup=qa/'preimages'/name
    backup.parent.mkdir(parents=True,exist_ok=True)
    if backup.exists():raise RuntimeError(f'Existing preimage {backup}')
    backup.write_bytes(source.read_bytes())
old_host=native/'craft_host.rs'
copy_record(old_host,native/'spatial_host.rs',lambda text:text.replace('craft_authoring','poet_spatial_authoring').replace('Craft.profile_sweep','ComputationalGeometry.profile_sweep').replace('Craft','spatial authoring').replace('extracted spatial authoring engine','Poet spatial authoring helpers'))
path=native/'geometry_host.rs';before=path.read_bytes()
text=before.decode('utf-8-sig').replace('craft_host','spatial_host').replace('craft_authoring','poet_spatial_authoring').replace('Craft.profile_sweep','ComputationalGeometry.profile_sweep').replace('craft_profile_grants','profile_grants')
after=text.encode();path.write_bytes(after)
rows.append(dict(source=str(path),target=str(path),source_sha256=hashlib.sha256(before).hexdigest(),target_sha256=hashlib.sha256(after).hexdigest()))

provenance=authoring/'provenance'
provenance.mkdir()
# The original notice remains historical evidence, not the adopted package licence.
for name in ['NOTICE','ATTRIBUTION.md']:
    copy_record(Path('C:/github/poet-cadcraft')/name,provenance/('original-'+name))
record=dict(saved_at_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    authority='Explicit human correction: MIT source libraries support existing Poet/Webizen/QualiaDB functionality; no integrated Craft suite, product, UI or capability namespace.',
    selected_license='MIT; original MIT copyright/permission text retained. Historical notice is provenance, not a dual-licence assertion for the new packages.',
    preserved=['C:/github/poet-cadcraft','C:/github/poet-*craft','C:/github/craft','native/src/graphical/craft_host.rs (unmounted pending documented cleanup)'],
    source_map=rows)
(provenance/'source-map.json').write_text(json.dumps(record,indent=2)+'\n')
(qa/'save-receipt.json').write_text(json.dumps(record,indent=2)+'\n')
print(f'Saved {len(rows)} source/provenance transitions and two internal Poet manifests. No original/staged sources removed.')
