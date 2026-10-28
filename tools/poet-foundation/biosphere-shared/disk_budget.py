"""Disk headroom for this workstream's owned Cargo processes, not a system quota."""
import os
import shutil
import subprocess
import sys
import time

GIB = 1024 ** 3
MIN_START_FREE = 30 * GIB
MIN_RUNNING_FREE = 8 * GIB
DISK_GUARD_EXIT = 74


def require_start_space(path):
    free = shutil.disk_usage(path).free
    if free < MIN_START_FREE:
        raise RuntimeError(
            f"Cargo not started: {free / GIB:.1f} GiB free; "
            f"at least {MIN_START_FREE / GIB:.0f} GiB required for build headroom"
        )
    return free


def stop_owned_process(child):
    """The caller retains the Popen handle for its own child throughout this call."""
    if child.poll() is not None:
        return
    if os.name == "nt":
        # Direct process-tree command with an owned numeric PID; no shell or file deletion.
        result = subprocess.run(
            ["taskkill.exe", "/PID", str(child.pid), "/T", "/F"],
            capture_output=True, text=True, check=False,
        )
        if result.returncode and child.poll() is None:
            raise RuntimeError("could not stop owned Cargo tree after disk guard fired")
    else:
        child.terminate()
    child.wait()


def wait_with_disk_budget(child, path):
    while True:
        code = child.poll()
        if code is not None:
            return code
        try:
            free = shutil.disk_usage(path).free
        except OSError:
            stop_owned_process(child)
            raise
        if free < MIN_RUNNING_FREE:
            print(
                f"disk guard: stopping owned Cargo tree; {free / GIB:.1f} GiB free "
                f"is below the {MIN_RUNNING_FREE / GIB:.0f} GiB running reserve; "
                "no successful qualification is claimed", file=sys.stderr, flush=True,
            )
            stop_owned_process(child)
            return DISK_GUARD_EXIT
        time.sleep(2)
