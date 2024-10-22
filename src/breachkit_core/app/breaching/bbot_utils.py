from __future__ import annotations
import subprocess
import asyncio

async def run_bbot_scan(target: str) -> str:
    """
    Run a BBot scan asynchronously on the target and return the output.
    """
    command = f"bbot scan {target} --json"
    process = await asyncio.create_subprocess_shell(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )
    stdout, stderr = await process.communicate()

    if process.returncode != 0:
        raise RuntimeError(f"BBot scan failed: {stderr.decode()}")

    return stdout.decode()
