from litestar import get, post
from breachkit.app.breaching.bbot_utils import run_bbot, parse_bbot_output
from pathlib import Path

@post("/recon/start")
async def start_recon(data: dict):
    target = data["target"]
    output_dir = Path("./data")
    run_bbot(target, output_dir)
    return {"message": "Recon started."}

@get("/recon/results")
async def get_recon_results():
    output_file = Path("./data/bbot_output.json")
    results = parse_bbot_output(output_file)
    return results
