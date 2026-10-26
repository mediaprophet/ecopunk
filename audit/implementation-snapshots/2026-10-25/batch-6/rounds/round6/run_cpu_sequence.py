from pathlib import Path
import subprocess,sys
p=Path(__file__).resolve().parent
for mode in ["surfaces","wire","catalog","hostoptin","workflow","intents","jobs","inspection"]:
    result=subprocess.run([sys.executable,str(p/"run_check.py"),mode])
    if result.returncode:sys.exit(result.returncode)
