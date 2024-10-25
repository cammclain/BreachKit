from __future__ import annotations
import subprocess
import json
from typing import Any, Dict, List

async def run_bbot_scan(target: str) -> Dict[str, Any]:
    """
    Asynchronously run a BBot scan on the specified target and return the parsed JSON output.

    Args:
        target (str): The target for the scan (e.g., domain, IP).

    Returns:
        Dict[str, Any]: Parsed JSON output from the BBot scan.
    """
    command = f"bbot scan {target} --json"
    process = await subprocess.create_subprocess_shell(
        command, stdout=subprocess.PIPE, stderr=subprocess.PIPE
    )
    stdout, stderr = await process.communicate()

    if process.returncode != 0:
        raise RuntimeError(f"BBot scan failed: {stderr.decode()}")

    return json.loads(stdout.decode())

def parse_bbot_results(results: Dict[str, Any]) -> List[Dict[str, str]]:
    """
    Parse the BBot scan results to extract relevant information.

    Args:
        results (Dict[str, Any]): Raw BBot scan output.

    Returns:
        List[Dict[str, str]]: List of relevant findings.
    """
    findings = []
    for item in results.get("findings", []):
        findings.append({
            "type": item.get("type", "Unknown"),
            "description": item.get("description", ""),
            "severity": item.get("severity", "info"),
        })
    return findings
