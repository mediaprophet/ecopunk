from pathlib import Path
import subprocess,sys
b=Path(__file__).resolve().parent
for mode in ['uvkernel','image','authoring','detail','atlas','integration','surfaces','wire','workflow','legacyworkflow','hmctextures','jobs','intents','inspection']:
    code=subprocess.call([sys.executable,str(b/'run_check.py'),mode])
    if code:raise SystemExit(code)
